def show_sprite(screen, sprite, x_offset, y_offset):
    pixels, mirror_x, mirror_y = sprite
    for y, row in enumerate(pixels):
        for x, c in enumerate(row):
            screen.pixel(x + x_offset, y + y_offset, c)
    m_offset_x = len(pixels[0])
    m_offset_y = len(pixels)
    if mirror_x > 1:
        m_offset_x -= 1
    if mirror_y > 1:
        m_offset_y -= 1
    if mirror_x:
        for y, row in enumerate(pixels):
            for x, c in enumerate(reversed(row)):
                screen.pixel(x + x_offset + m_offset_x, y + y_offset, c)
    if mirror_y:
        for y, row in enumerate(reversed(pixels)):
            for x, c in enumerate(row):
                screen.pixel(x + x_offset, y + y_offset + m_offset_y, c)
    if mirror_x and mirror_y:
        for y, row in enumerate(reversed(pixels)):
            for x, c in enumerate(reversed(row)):
                screen.pixel(x + x_offset + m_offset_x, y + y_offset + m_offset_y, c)


def show_digit(screen, digit, x_offset, y_offset):
    sprite = globals()["DIGIT_" + str(digit)]
    for segment in sprite:
        pixels, segment_x, segment_y = segment
        for y, row in enumerate(pixels):
            for x, c, in enumerate(row):
                if c:
                    screen.pixel(x + x_offset + segment_x, y + y_offset + segment_y, c)


def mirror_x(array):
    return [list(reversed(row)) for row in array]


LOGO = (
    [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ],
    1,  # mirror x
    1,  # mirror y
)

GRAM = (
    [
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 1, 1, 0, 0, 0],
        [1, 0, 0, 0, 0, 1, 1, 1, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0, 1, 0],
        [1, 1, 0, 0, 0, 0, 0, 0, 1, 1],
        [1, 1, 0, 0, 0, 0, 0, 0, 1, 1],
        [1, 1, 0, 0, 0, 0, 0, 0, 1, 1],
        [1, 1, 0, 0, 0, 0, 0, 0, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    ],
    0,
    0,
)

BATTERY = (
    [
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1],
    ],
    0,
    2,  # mirror y but overlap 1 pixel
)

DOT = ([[1, 1], [1, 1]], 1, 1)

SEGMENT_1 = (
    [[1, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 0, 1, 1, 1, 1, 1, 1], [0, 0, 0, 1, 1, 1, 1, 1]],
    1,  # offset x
    0,  # offset y
)

SEGMENT_2 = (mirror_x(SEGMENT_1[0]), 10, 0)

SEGMENT_3 = (
    [
        [1, 0, 0, 0],
        [1, 1, 0, 0],
        [1, 1, 1, 0],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [0, 1, 1, 1],
        [0, 0, 1, 1],
    ],
    0,
    1,
)

SEGMENT_4 = (mirror_x(SEGMENT_3[0]), 15, 1)

SEGMENT_5 = ([[1, 0], [1, 1], [1, 1], [1, 0]], 0, 13)

SEGMENT_6 = (
    [
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    ],
    4,
    13,
)

SEGMENT_7 = (mirror_x(SEGMENT_5[0]), 17, 13)

SEGMENT_8 = (list(reversed(SEGMENT_3[0])), 0, 16)

SEGMENT_9 = (mirror_x(SEGMENT_8[0]), 15, 16)

SEGMENT_10 = (list(reversed(SEGMENT_1[0])), 1, 26)

SEGMENT_11 = (mirror_x(SEGMENT_10[0]), 10, 26)

DIGIT_MINUS = [SEGMENT_6]

DIGIT_0 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_3,
    SEGMENT_4,
    SEGMENT_5,
    SEGMENT_7,
    SEGMENT_8,
    SEGMENT_9,
    SEGMENT_10,
    SEGMENT_11,
]

DIGIT_1 = [
    SEGMENT_4,
    SEGMENT_7,
    SEGMENT_9,
]

DIGIT_2 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_4,
    SEGMENT_6,
    SEGMENT_8,
    SEGMENT_10,
    SEGMENT_11,
]

DIGIT_3 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_4,
    SEGMENT_6,
    SEGMENT_7,
    SEGMENT_9,
    SEGMENT_10,
    SEGMENT_11,
]

DIGIT_4 = [
    SEGMENT_3,
    SEGMENT_4,
    SEGMENT_6,
    SEGMENT_7,
    SEGMENT_9,
]

DIGIT_5 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_3,
    SEGMENT_6,
    SEGMENT_9,
    SEGMENT_10,
    SEGMENT_11,
]

DIGIT_6 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_3,
    SEGMENT_5,
    SEGMENT_6,
    SEGMENT_8,
    SEGMENT_9,
    SEGMENT_10,
    SEGMENT_11,
]

DIGIT_7 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_3,
    SEGMENT_4,
    SEGMENT_7,
    SEGMENT_9,
]

DIGIT_8 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_3,
    SEGMENT_4,
    SEGMENT_5,
    SEGMENT_6,
    SEGMENT_7,
    SEGMENT_8,
    SEGMENT_9,
    SEGMENT_10,
    SEGMENT_11,
]

DIGIT_9 = [
    SEGMENT_1,
    SEGMENT_2,
    SEGMENT_3,
    SEGMENT_4,
    SEGMENT_6,
    SEGMENT_7,
    SEGMENT_9,
    SEGMENT_10,
    SEGMENT_11,
]
