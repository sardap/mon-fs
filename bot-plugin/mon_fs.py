from functools import cached_property
import json
from pathlib import Path
from typing import Dict, Generator, Iterable, List, Optional, Tuple, Union
from modules.context import context
from modules.battle import BattleOutcome
from modules.items import Item, get_item_bag, get_item_by_index, get_item_by_name, get_item_storage
from modules.memory import GameState, get_game_state, read_symbol, unpack_uint16, unpack_uint32
from modules.menu_parsers import get_cursor_options, parse_menu
from modules.menuing import scroll_to_item_in_bag
from modules.modes._interface import BotListener, BotMode, BotModeError, FrameInfo
from modules.modes.spin import SpinMode
from modules.player import RunningState, TileTransitionState, get_player_avatar
from modules.player_pc_navigaton import DepositItemNavigator, WithdrawItemNavigator
from modules.plugin_interface import BotPlugin
from modules.pokemon import Pokemon, get_opponent
from modules.pokemon_storage import PokemonStorageSlot, get_pokemon_storage
from modules.pokemon_storage_navigaton import BoxNavigator, PCMainMenuNavigator, StorageChangeScreen, StorageCursor, get_storage_screen, get_storage_state
from modules.profiles import Profile
from modules.tasks import task_is_active


class BoxMon:
    def __init__(self, species: str, gender: str, name: str, held_item: str) -> None:
        self.species = species.capitalize()
        self.gender = gender.lower()
        self.name = name
        self.caught_string =f"{self.species}:{self.gender}:{self.name}"
        if held_item  == "":
            self.held_item = None
        else:
            self.held_item = get_item_by_name(held_item)

    def __dict__(self):
        return {
            "BoxMon": {
                "species": self.species,
                "gender": self.gender,
                "name": self.name,
                "held_item": self.held_item.name,
            }
        }


def pokemon_caught_string(pokemon: Pokemon):
    return f"{pokemon.species.name}:{pokemon.gender}:{pokemon.name}"

class Guide:
    def __init__(self, mons: List[BoxMon]) -> None:
        self.mons = mons
        self.caught_min: Dict[str, int] = {}
        for mon in self.mons:
            if mon.caught_string not in self.caught_min:
                self.caught_min[mon.caught_string] = 0

            self.caught_min[mon.caught_string] += 1


    @cached_property
    def have_caught(self) -> Dict[str, int]:
        have_caught = self.caught_min.copy()

        storage = get_pokemon_storage()

        for box in storage.boxes_including_empty:
            for slot in box.slots:
                catch_string = pokemon_caught_string(slot.pokemon)

                if have_caught.get(catch_string, 0) > 0:
                    have_caught[catch_string] -= 1


        return have_caught

    def need_to_catch_any(self) -> bool:
        return any(self.have_caught.values())

    @property
    def percent_complete(self) -> float:
        return 1 - sum(self.have_caught.values()) / len(self.mons)

    def is_marker_mon_candiate(self, mon: Pokemon) -> bool:
        return mon.species.name not in self.marker_mon_blocks
        

    def need(
        self, pokemon: Pokemon
    ) -> Union[BoxMon, None]:
        for mon in self.mons:
            if self.have_caught.get(mon.caught_string, 0) == 0:
                continue

            if mon.species == pokemon.species.name and mon.gender == pokemon.gender:
                return mon
            
        return None


    def storage_mons_in_wrong_position(self) -> List[Tuple[int, int]]:
        storage = get_pokemon_storage()

        wrong_positions = []

        for i, mon in enumerate(self.mons):
            box_number = i // BOX_SIZE
            box_index = i % BOX_SIZE
            
            storage_mon = storage.boxes_including_empty[box_number].slots[box_index].pokemon
            if not (mon.name == storage_mon.name and mon.species == storage_mon.species.name and mon.gender == storage_mon.gender):
                wrong_positions.append((box_number, box_index))
                continue

        return wrong_positions
    
    def non_guide_mons_present_in_pc(self) -> bool:
        storage = get_pokemon_storage()

        for i in range(BOX_COUNT):
            for j in range(BOX_SIZE):
                storage_mon = storage.boxes_including_empty[i].slots[j].pokemon
                if storage_mon.is_empty:
                    continue

                catch_str = pokemon_caught_string(storage_mon)
                if catch_str not in self.have_caught:
                    return True
                
        return False
    
    def storage_mons_missing_items(self) -> List[Tuple[int, int]]:
        storage = get_pokemon_storage()

        missing_items = []

        for i, mon in enumerate(self.mons):
            box_number = i // BOX_SIZE
            box_index = i % BOX_SIZE
            

            storage_mon = storage.boxes_including_empty[box_number].slots[box_index].pokemon
            if storage_mon.held_item != mon.held_item:
                missing_items.append((box_number, box_index))

        return missing_items

guide: Optional[Guide] = None


def load_pc_guide_dict(path: Path):
    global guide
    mons = []

    with open(path / "pc.json", "r") as f:
        raw_dict = json.load(f)

    for mon in raw_dict["mons"]:
        if mon is None:
            continue
        else:
            mons.append(
                BoxMon(
                    mon["species"],
                    mon["gender"],
                    mon["name"],
                    mon["held_item"],
                )
            )

    guide = Guide(mons)




class RequiredItems:
    def __init__(self, item: Item, quantity: int) -> None:
        self.item = item
        self.quantity = quantity

class MissingRequiredItemError(ValueError):
    def __init__(self, items: List[RequiredItems]) -> None:
        self.item = items
        super().__init__(f"Missing required item {','.join([f"{item.item.name}:{item.quantity}" for item in items])} in bag or storage")


class GrabDataItemsFromPC(BotMode):
    @staticmethod
    def name() -> str:
        return "mon-fs: grab required items from PC"
    
    @staticmethod
    def is_selectable() -> bool:
        return True


    def run(self) -> Generator:
        required_items: Dict[Item, int] = {}

        free_item_slots = 30 - len(get_item_bag().items)

        storage = get_pokemon_storage()
        for i, mon in enumerate(guide.mons):
            if mon.held_item is None:
                continue

            # Dont count the item if the mon is already holding it
            box_number = i // BOX_SIZE
            box_index = i % BOX_SIZE
            storage_mon = storage.boxes_including_empty[box_number].slots[box_index].pokemon
            if storage_mon.held_item != mon.held_item:
                if mon.held_item not in required_items:
                    if len(required_items) + 1 > free_item_slots:
                        continue
                    required_items[mon.held_item] = 0
                required_items[mon.held_item] += 1


        items = [RequiredItems(item, quantity) for item, quantity in required_items.items()]
        items.sort(key=lambda x: x.item.index)

        missing_items = []
        # Ensure we have all the required items before proceeding
        for required_item in items:
            bag_qauntity = get_item_bag().quantity_of(required_item.item)
            storage_quantity = get_item_storage().quantity_of(required_item.item)
            if bag_qauntity + storage_quantity < required_item.quantity:
                missing_items.append(required_item)

        if missing_items:
            raise MissingRequiredItemError(missing_items)
        
        for required_item in items:
            while get_item_bag().quantity_of(required_item.item) < required_item.quantity:
                yield from WithdrawItemNavigator(required_item.item, required_item.quantity).step()

        raise BotModeError("All required items have been given to box mons")


class DepositBackToItemsToPC(BotMode):
    @staticmethod
    def name() -> str:
        return "mon-fs: deposit items back to to PC"
    
    @staticmethod
    def is_selectable() -> bool:
        return True
    
    def run(self) -> Generator:
        for slot in get_item_bag().items:
            if slot is None:
                continue

            if get_item_bag().quantity_of(slot.item) == 0:
                continue

            yield from DepositItemNavigator(slot.item, 0).step()

            if len(get_item_bag().items) == 0:
                break
        

BOX_COUNT = 14
BOX_SIZE = 30
BOX_WIDTH = 6
BOX_HEIGHT = 5

class GiveItemsToBoxMons(BotMode):
    @staticmethod
    def name() -> str:
        return "mon-fs: give items to box mons"
    
    @staticmethod
    def is_selectable() -> bool:
        return len(guide.storage_mons_in_wrong_position()) == 0 and len(guide.storage_mons_missing_items()) > 0
    
    def run(self) -> Generator:
        while get_game_state() != GameState.POKE_STORAGE:
            yield from PCMainMenuNavigator("MOVE_ITEMS").step()

        for i, mon in enumerate(guide.mons):
            # Find the box number
            box_number = i // BOX_SIZE
            box_index = i % BOX_SIZE
            box_x = box_index % BOX_WIDTH
            box_y = box_index // BOX_WIDTH

            storage_mon = get_pokemon_storage().boxes_including_empty[box_number].slots[box_index].pokemon

            if storage_mon.held_item == mon.held_item:
                continue

            if storage_mon.held_item is not None:
                # Take the current item from the box mon
                yield from BoxNavigator((box_x, box_y), box_number, "TAKE").step()
                yield from bad_wait()
                context.emulator.press_button("B")
                yield from bad_wait()
                yield from bad_wait()
                context.emulator.press_button("A")
                yield from bad_wait()
                yield from bad_wait()

            if mon.held_item is None or get_item_bag().quantity_of(mon.held_item) == 0:
                continue

            while get_storage_screen() != StorageChangeScreen.ItemFromBag:
                yield from BoxNavigator((box_x, box_y), box_number, "GIVE").step()
                while unpack_uint16(read_symbol("gPaletteFade", offset=0x07, size=0x02)) & 0x80 != 0:
                    yield

            while get_game_state() != GameState.BAG_MENU:
                yield

            yield from scroll_to_item_in_bag(mon.held_item)

            while get_game_state() != GameState.POKE_STORAGE:
                context.emulator.press_button("A")
                yield

            while not task_is_active("TASK_RESHOWPOKESTORAGE"):
                yield

            while get_storage_state() != 2:
                yield

            while get_storage_state() == 2:
                context.emulator.press_button("A")
                yield

        raise BotModeError("All items have been given to box mons")


class ScreenshotMode(BotMode):
    @staticmethod
    def name() -> str:
        return "mon-fs: screenshot"
    
    @staticmethod
    def is_selectable() -> bool:
        return len(guide.storage_mons_in_wrong_position()) == 0 and len(guide.storage_mons_missing_items()) == 0
    
    def run(self) -> Generator:
        while get_game_state() != GameState.POKE_STORAGE:
            yield from PCMainMenuNavigator("MOVE_POKEMON").step()

        for i in range(len(guide.mons)):
            # Find the box number
            box_number = i // BOX_SIZE
            box_index = i % BOX_SIZE
            box_x = box_index % BOX_WIDTH
            box_y = box_index // BOX_WIDTH

            yield from BoxNavigator((box_x, box_y), box_number, None).step()

            for _ in range(100):
                yield

            context.emulator.take_screenshot(f"mon_fs_{i}")

        raise BotModeError("All items have been given to box mons")


class ClearPC(BotMode):
    @staticmethod
    def name() -> str:
        return "mon-fs: Remove non data mons from PC"
    
    @staticmethod
    def is_selectable() -> bool:
        return guide.non_guide_mons_present_in_pc()
    
    def run(self) -> Generator:
        while get_game_state() != GameState.POKE_STORAGE:
            yield from PCMainMenuNavigator("MOVE_POKEMON").step()
            yield

        for box_number in range(BOX_COUNT):
            for box_index in range(BOX_SIZE):

                box_x = box_index % BOX_WIDTH
                box_y = box_index // BOX_WIDTH

                storage_mon = get_pokemon_storage().boxes_including_empty[box_number].slots[box_index].pokemon
                if storage_mon.is_empty:
                    continue

                if pokemon_caught_string(storage_mon) in guide.have_caught:
                    continue

                yield from BoxNavigator((box_x, box_y), box_number, "RELEASE").step()

        raise BotModeError("PC has been cleared")


def get_mon_current_location(mon: BoxMon) -> Optional[Tuple[int, int]]:
    # Look foward to find the correct mon
    catch_str = mon.caught_string

    for i in range(BOX_COUNT):
        for j in range(BOX_SIZE):
            other_mon = get_pokemon_storage().boxes_including_empty[i].slots[j].pokemon
            if pokemon_caught_string(other_mon) == catch_str:
                return (i, j)
            
    return None
    

def get_mon_epxected_location(mon: BoxMon) -> Tuple[int, int]:
    index = guide.mons.index(mon)

    box_number = index // BOX_SIZE
    box_index = index % BOX_SIZE

    return (box_number, box_index)

def get_mon_index_from_catch_string(catch_str: str) -> Optional[BoxMon]:
    for mon in guide.mons:
        if mon.caught_string == catch_str:
            return mon

    return None


def bad_wait():
    for _ in range(20):
        yield

def put_mon_in_place(mon: BoxMon) -> Generator:
    current_location = get_mon_current_location(mon)
    if current_location is not None:
        current_box_number = current_location[0]
        current_box_index = current_location[1]
        current_box_x = current_box_index % BOX_WIDTH
        current_box_y = current_box_index // BOX_WIDTH

    expected_location = get_mon_epxected_location(mon)
    expected_box_number = expected_location[0]
    expected_box_index = expected_location[1]
    expected_box_x = expected_box_index % BOX_WIDTH
    expected_box_y = expected_box_index // BOX_WIDTH

    mon_in_expected_location = get_mon_index_from_catch_string(pokemon_caught_string(get_pokemon_storage().boxes_including_empty[expected_box_number].slots[expected_box_index].pokemon))

    if mon_in_expected_location == mon:
        return

    if current_location is not None:
        yield from BoxNavigator((current_box_x, current_box_y), current_box_number, "MOVE").step()
        yield from bad_wait()

    yield from BoxNavigator((expected_box_x, expected_box_y), expected_box_number, None).step()
    yield from bad_wait()
        
    context.emulator.press_button("A")
    yield from bad_wait()

    context.emulator.press_button("A")
    yield from bad_wait()
    yield from bad_wait()

    if mon_in_expected_location is None:
        return
    
    yield from put_mon_in_place(mon_in_expected_location)


class SortPC(BotMode):
    @staticmethod
    def name() -> str:
        return "mon-fs: sort PC"
    
    @staticmethod
    def is_selectable() -> bool:
        return not guide.need_to_catch_any() and len(guide.storage_mons_in_wrong_position()) > 0
    
    def run(self) -> Generator:
        while get_game_state() != GameState.POKE_STORAGE:
            yield from PCMainMenuNavigator("MOVE_POKEMON").step()

        for box_number in range(BOX_COUNT):
            for box_index in range(BOX_SIZE):
                index = box_number * BOX_SIZE + box_index

                if index >= len(guide.mons):
                    raise BotModeError("PC has been sorted")

                mon = guide.mons[index]

                yield from put_mon_in_place(mon)


class MonFsSpinMode(BotMode):
    @staticmethod
    def name() -> str:
        return "mon-fs: Spin"

    @staticmethod
    def is_selectable() -> bool:
        return get_player_avatar().map_location.has_encounters and guide.need_to_catch_any()

    def run(self) -> Generator:
        directions = ["Up", "Right", "Down", "Left"]

        while guide.need_to_catch_any():
            avatar = get_player_avatar()
            if (
                get_game_state() == GameState.OVERWORLD
                and avatar.tile_transition_state == TileTransitionState.NOT_MOVING
                and avatar.running_state == RunningState.NOT_MOVING
            ):
                direction_index = (directions.index(avatar.facing_direction) + 1) % len(directions)
                context.emulator.press_button(directions[direction_index])
            yield

        raise BotModeError("All required mons have been caught")


class MonFsPlugin(BotPlugin):
    @staticmethod
    def name() -> str:
        return "Mon FS"

    def __init__(self) -> None:
        super().__init__()

    def get_additional_bot_modes(self) -> Iterable[BotMode]:
        return [MonFsSpinMode, ScreenshotMode, GiveItemsToBoxMons, SortPC, GrabDataItemsFromPC, DepositBackToItemsToPC, ClearPC]

    def on_profile_loaded(self, profile: Profile) -> None:
        load_pc_guide_dict(profile.path)

    def custom_catch_filters(self, opponent: Pokemon) -> None:
        need = guide.need(opponent)
        if need:
            return "mon fs"
        return None
    
    def on_battle_started(self, opponent: Pokemon) -> None:
        from modules.context import context

        need = guide.need(opponent)
        if need:
            context.next_name = need.name
        else:
            context.next_name = "5vddqqaMXN"

    def on_battle_ended(self, outcome: BattleOutcome) -> None:
        if outcome == BattleOutcome.Caught:
            del guide.have_caught
            guide.have_caught
            print(f"Percent complete: {guide.percent_complete * 100:.2f}%")
