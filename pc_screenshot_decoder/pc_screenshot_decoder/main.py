import natsort
import os
import cv2
import numpy as np
from cv2.typing import MatLike
from pydantic import BaseModel
from PIL import Image
from typing import List, Optional
from hashlib import md5
from shutil import rmtree

SCREENSHOT_FOLDER = os.getenv("PC_DEC_SCREENSHOT_FOLDER", "./example/photo")

WORKING_FOLDER = os.getenv("PC_DEC_WORKING_FOLDER", "./working")
FONTS_INPUT_FOLDER = os.getenv("PC_DEC_FONTS_FOLDER", "./fonts/input")
OUTPUT_FONTS_FOLDER = os.getenv("PC_DEC_OUTPUT_FONTS_FOLDER", "./fonts/letters")

GENDER_FOLDER = os.path.join(OUTPUT_FONTS_FOLDER, "genders")


class SpriteSheet(BaseModel):
    image_path: str
    sprite_width: int
    sprite_height: int
    output_folder: str

    def split(self):
        split_spritesheet(
            self.image_path, self.sprite_width, self.sprite_height, self.output_folder
        )

    def exists(self):
        return os.path.exists(self.output_folder)


NUMBER_OFFSET = 70
UPPERCASE_OFFSET = 96
LOWERCASE_OFFSET = 122

# #9496ad
PC_BACKGROUND_COLOR = (148, 150, 173)

# #ffffff
PC_TEXT_COLOR = (255, 255, 255)

# #000000
PC_TEXT_SHADOW_COLOR = (0, 0, 0)

SPITE_SHEET_BACKGROUND_COLOR = (144, 200, 255)


def crop_sprite(sprite: Image) -> Image:
    sprite = sprite.convert("RGB")

    right = sprite.width
    top = 0
    lower = sprite.height

    # Start at lower and move up until we find a non light blue pixel
    for lower in range(sprite.height - 1, -1, -1):
        pixel = sprite.getpixel((0, lower))
        if pixel != SPITE_SHEET_BACKGROUND_COLOR:
            break

    for top in range(0, sprite.height):
        pixel = sprite.getpixel((0, top))
        if pixel != SPITE_SHEET_BACKGROUND_COLOR:
            break

    # Start at right and move left until we find a non light blue pixel
    for right in range(sprite.width - 1, -1, -1):
        pixel = sprite.getpixel((right, lower))
        if pixel != SPITE_SHEET_BACKGROUND_COLOR:
            break

    if right == 0 or lower == 0:
        return Image.new("RGBA", (0, 0))

    # Fucked up some math by 1 just add one ez dog
    return sprite.crop((0, top, right + 1, lower))


def split_spritesheet(
    image_path: str, sprite_width: int, sprite_height: int, output_folder: str
):
    img = Image.open(image_path)

    sheet_width, sheet_height = img.size

    sprites_across = sheet_width // sprite_width
    sprites_down = sheet_height // sprite_height

    try:
        rmtree(output_folder)
    except FileNotFoundError:
        pass

    os.makedirs(output_folder, exist_ok=True)

    sprite_number = 0

    for y in range(sprites_down):
        for x in range(sprites_across):
            left = x * sprite_width
            upper = y * sprite_height
            right = left + sprite_width
            lower = upper + sprite_height

            sprite = img.crop((left, upper, right, lower))

            sprite = sprite.convert("RGBA")

            data = np.array(sprite)

            r, g, b, _a = data.T
            white_areas = (r == 255) & (g == 255) & (b == 255)
            data[..., :-1][white_areas.T] = PC_BACKGROUND_COLOR
            text_areas = (r == 56) & (g == 56) & (b == 56)
            data[..., :-1][text_areas.T] = PC_TEXT_COLOR
            text_shadow_areas = (r == 216) & (g == 216) & (b == 216)
            data[..., :-1][text_shadow_areas.T] = PC_TEXT_SHADOW_COLOR

            sprite = Image.fromarray(data)

            sprite = crop_sprite(sprite)

            sprite = sprite.convert("RGBA")
            data = np.array(sprite)

            r, g, b, a = data.T
            background_areas = (
                (r == PC_BACKGROUND_COLOR[0])
                & (g == PC_BACKGROUND_COLOR[1])
                & (b == PC_BACKGROUND_COLOR[2])
            )
            data[..., :-1][background_areas.T] = (0, 0, 0)
            if np.all(data[..., :-1] == 0):
                continue

            percentage = 1
            sprite = sprite.resize(
                (sprite.width * percentage, sprite.height * percentage), Image.NEAREST
            )

            if sprite_number >= NUMBER_OFFSET and sprite_number < UPPERCASE_OFFSET:
                number = sprite_number - NUMBER_OFFSET
                sprite.save(os.path.join(output_folder, f"{number}.png"))

            if sprite_number >= UPPERCASE_OFFSET and sprite_number < LOWERCASE_OFFSET:
                uppercase = sprite_number - UPPERCASE_OFFSET
                sprite.save(
                    os.path.join(output_folder, f"{chr(uppercase + 65).upper()}.png")
                )

            if (
                sprite_number >= LOWERCASE_OFFSET
                and sprite_number < LOWERCASE_OFFSET + 26
            ):
                lowercase = sprite_number - LOWERCASE_OFFSET
                sprite.save(
                    os.path.join(output_folder, f"{chr(lowercase + 65).lower()}.png")
                )

            sprite_number += 1


SHEETS = [
    SpriteSheet(
        image_path=os.path.join(FONTS_INPUT_FOLDER, "latin_normal.png"),
        sprite_width=16,
        sprite_height=16,
        output_folder=os.path.join(OUTPUT_FONTS_FOLDER, "latin_normal"),
    ),
    SpriteSheet(
        image_path=os.path.join(FONTS_INPUT_FOLDER, "latin_short.png"),
        sprite_width=16,
        sprite_height=16,
        output_folder=os.path.join(OUTPUT_FONTS_FOLDER, "latin_short"),
    ),
    SpriteSheet(
        image_path=os.path.join(FONTS_INPUT_FOLDER, "latin_small_narrow.png"),
        sprite_width=16,
        sprite_height=16,
        output_folder=os.path.join(OUTPUT_FONTS_FOLDER, "latin_small_narrow"),
    ),
    SpriteSheet(
        image_path=os.path.join(FONTS_INPUT_FOLDER, "latin_small.png"),
        sprite_width=16,
        sprite_height=16,
        output_folder=os.path.join(OUTPUT_FONTS_FOLDER, "latin_small"),
    ),
]

NORMAL_LETTERS = SHEETS[0]
SHORT_LETTERS = SHEETS[1]
SMALL_NARROW_LETTERS = SHEETS[2]
SMALL_LETTERS = SHEETS[3]


class Letter(BaseModel):
    letter: str
    path: str
    color: tuple[int, int, int]


def letter_color(letter: str) -> tuple[int, int, int]:
    md5_hash = md5(letter.encode()).digest()
    val = int.from_bytes(md5_hash, "big")
    return (val & 0xFF, (val >> 8) & 0xFF, (val >> 16) & 0xFF)


def letters_from_spritesheet(
    sprite_sheet_folder: str, include_only_letters: Optional[str]
) -> List[Letter]:
    letters = []
    for file in os.listdir(sprite_sheet_folder):
        if file.endswith(".png"):
            letter = file.split(".")[0]
            match letter:
                case "10":
                    letter = "!"
                case "11":
                    letter = "?"
                case "12":
                    letter = "."
                case "13":
                    letter = "-"
                case "15":
                    letter = "…"
                case "20":
                    letter = "♂"
                case "21":
                    letter = "♀"
                case "23":
                    letter = ","
                case "24":
                    letter = "X"
                case "25":
                    letter = "/"
            if include_only_letters is not None and letter not in include_only_letters:
                continue
            path = os.path.join(sprite_sheet_folder, file)
            # Hash letter to get color
            color = letter_color(letter)
            letters.append(Letter(letter=letter, path=path, color=color))
    return letters


def possible_species_letters() -> str:
    species = [
        "POOCHYENA",
        "NINCADA",
        "WHISMUR",
        "TAILLOW",
    ]
    # get all unique letters
    result = ""
    letters = set()
    for specie in species:
        for letter in specie:
            letters.add(letter)
            result += letter
    return result


POSSIBLE_NAME_LETTERS = (
    "aAbBcCdDeEfFgGhHiIjJkKmMnNoOpPqQrRsStTuUvVwWxXyYzZ23456789!?/-…♂♀"
)


class Match(BaseModel):
    letter: str
    x: int
    y: int
    width: int
    height: int
    confidence: float


def match_template(
    search_img: MatLike,
    template: MatLike,
    letter: str,
) -> List[Match]:
    w, h = template.shape[::-1]
    res = cv2.matchTemplate(search_img, template, cv2.TM_CCOEFF_NORMED)
    threshold = 0.9
    loc = np.where(res >= threshold)
    matches: List[Match] = []
    for pt in zip(*loc[::-1]):
        matches.append(
            Match(
                letter=letter,
                x=pt[0],
                y=pt[1],
                width=w,
                height=h,
                confidence=res[pt[1], pt[0]],
            )
        )
    return matches


def find_letter(letter: Letter, search_img: MatLike) -> List[Match]:
    template = cv2.imread(letter.path, cv2.IMREAD_GRAYSCALE)
    assert template is not None, "file could not be read, check with os.path.exists()"

    return match_template(search_img, template, letter.letter)


def image_to_text(
    img_rgb: MatLike,
    sheet: SpriteSheet,
    include_only_letters: Optional[str] = None,
) -> Optional[str]:
    img_gray = cv2.cvtColor(img_rgb, cv2.COLOR_BGR2GRAY)

    letters = letters_from_spritesheet(sheet.output_folder, include_only_letters)

    matches: List[Match] = []

    for letter in letters:
        matches.extend(find_letter(letter, img_gray))

    matches.sort(key=lambda i: (i.x, i.y))

    if len(matches) == 0:
        return

    # This is always fucking me
    match_groups = {}
    for match in matches:
        group = match.x // 4
        if group not in match_groups:
            match_groups[group] = match
        elif match.confidence > match_groups[group].confidence:
            match_groups[group] = match

    matches = list(match_groups.values())

    result = ""
    for match in matches:
        cv2.rectangle(
            img_rgb,
            (match.x, match.y),
            (match.x + match.width, match.y + match.height),
            letter_color(match.letter),
            1,
        )
        result += match.letter

    return result


def recognize_gender(img_rgb: MatLike) -> Optional[str]:
    male = cv2.imread(os.path.join(GENDER_FOLDER, "male.png"), cv2.IMREAD_GRAYSCALE)

    img_gray = cv2.cvtColor(img_rgb, cv2.COLOR_BGR2GRAY)

    matches = match_template(img_gray, male, "M")
    if len(matches) > 0:
        cv2.rectangle(
            img_rgb,
            (matches[0].x, matches[0].y),
            (matches[0].x + matches[0].width, matches[0].y + matches[0].height),
            (255, 0, 0),
            1,
        )
        return "M"

    female = cv2.imread(os.path.join(GENDER_FOLDER, "female.png"), cv2.IMREAD_GRAYSCALE)

    matches = match_template(img_gray, female, "F")
    if len(matches) > 0:
        cv2.rectangle(
            img_rgb,
            (matches[0].x, matches[0].y),
            (matches[0].x + matches[0].width, matches[0].y + matches[0].height),
            (0, 0, 255),
            1,
        )
        return "F"

    return None


class BoxMon(BaseModel):
    name: str
    species: str
    gender: str
    item: str


def remove_consecutive_duplicates_until_n(s: str, n: int):
    for i in range(1, len(s)):
        if s[i] == s[i - 1]:
            s = s[:i] + s[i + 1 :]
            if len(s) == n:
                return s
            return remove_consecutive_duplicates_until_n(s, n)

    return s


def read_image(img_rgb: MatLike, working_name: Optional[str] = None) -> BoxMon:
    def write_to_working_folder():
        if WORKING_FOLDER is not None and working_name is not None:
            cv2.imwrite(
                os.path.join(WORKING_FOLDER, f"{working_name}_res.png"), img_rgb
            )

    name_start_y = 84 / 160
    name_end_y = 103 / 160
    name_part = img_rgb[
        int(img_rgb.shape[0] * name_start_y) : int(img_rgb.shape[0] * name_end_y),
        0 : img_rgb.shape[1],
    ]
    name = image_to_text(name_part, NORMAL_LETTERS, POSSIBLE_NAME_LETTERS)
    write_to_working_folder()
    assert name is not None, "Name could not be read"
    # A Full on shit hack
    if len(name) > 10:
        name = remove_consecutive_duplicates_until_n(name, 10)
    assert len(name) == 10, f"Expected 10 characters, got {name}({len(name)})"

    species_start_y = 103 / 160
    species_end_y = 117 / 160
    species_part = img_rgb[
        int(img_rgb.shape[0] * species_start_y) : int(img_rgb.shape[0] * species_end_y),
        0 : img_rgb.shape[1],
    ]
    species = image_to_text(species_part, SHORT_LETTERS, possible_species_letters())
    write_to_working_folder()

    gender_start_y = 115 / 160
    gender_end_y = 134 / 160
    gender_lvl_part = img_rgb[
        int(img_rgb.shape[0] * gender_start_y) : int(img_rgb.shape[0] * gender_end_y),
        0 : img_rgb.shape[1],
    ]
    gender = recognize_gender(gender_lvl_part)
    write_to_working_folder()

    item_start_y = 132 / 160
    item_part = img_rgb[int(img_rgb.shape[0] * item_start_y) :, 0 : img_rgb.shape[1]]
    item = image_to_text(item_part, SMALL_NARROW_LETTERS)
    write_to_working_folder()
    if item is None:
        item = ""

    mon = BoxMon(name=name, species=species, gender=gender, item=item)

    return mon


class Result(BaseModel):
    boxes: List[List[BoxMon]]


def camera_test():
    cam = cv2.VideoCapture(0)

    # Get the default frame width and height
    frame_width = int(cam.get(cv2.CAP_PROP_FRAME_WIDTH))
    frame_height = int(cam.get(cv2.CAP_PROP_FRAME_HEIGHT))

    # Define the codec and create VideoWriter object
    fourcc = cv2.VideoWriter_fourcc(*"mp4v")
    out = cv2.VideoWriter("output.mp4", fourcc, 20.0, (frame_width, frame_height))

    while True:
        ret, frame = cam.read()

        # Write the frame to the output file
        out.write(frame)

        # Display the captured frame
        cv2.imshow("Camera", frame)

        # Press 'q' to exit the loop
        key = cv2.waitKey(1)
        if key == ord("q"):
            break
        elif key == ord(" "):
            break

        try:
            x = read_image(frame)
            print(x)
        except Exception as e:
            pass

    # Release the capture and writer objects
    cam.release()
    out.release()
    cv2.destroyAllWindows()


def main():
    # Make sure all the sprite sheets are split
    for sheet in SHEETS:
        if not sheet.exists():
            sheet.split()

    # camera_test()

    result = Result(boxes=[[]])

    if os.path.exists(WORKING_FOLDER):
        rmtree(WORKING_FOLDER)

    os.makedirs(WORKING_FOLDER, exist_ok=True)

    files = os.listdir(SCREENSHOT_FOLDER)
    files = natsort.natsorted(files)
    # read images in SCREENSHOT_FOLDER
    for screenshot in files:
        if screenshot.endswith(".png"):
            path = os.path.join(SCREENSHOT_FOLDER, screenshot)

            img_rgb = cv2.imread(path)
            assert img_rgb is not None, "file could not be read"

            striped_file_name = os.path.splitext(
                os.path.basename(os.path.normpath(path))
            )[0]

            mon = read_image(img_rgb, striped_file_name)
            if len(result.boxes[-1]) + 1 > 30:
                result.boxes.append([])
            result.boxes[-1].append(mon)

    print(result.model_dump_json())


if __name__ == "__main__":
    main()
