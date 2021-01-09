import sys
import json
from math import sqrt
from random import choice, random, uniform

# A simple script to generate a random "big scene"
# You must provide a path for the output file as a command-line argument

def center_ok(center):
    big_centers = ([4.0, 1.0, 0.0], [0.0, 1.0, 0.0], [-4.0, 1.0, 0.0])
    for other in big_centers:
        v = (center[0] - other[0], center[1] - other[1], center[2] - other[2])
        mag = sqrt(sum(x ** 2 for x in v))
        if mag < 1.5: return False
    return True

def random_color():
    return [min(255, int(random() * 255.0)) for _ in range(3)]

def random_lambertian():
    return {"type": "lambertian", "color": random_color()}

def random_glass():
    return {"type": "glass", "color": random_color(), "refraction_index": uniform(1.0, 2.0)}

def random_metal():
    return {"type": "metal", "color": random_color(), "fuzziness": random()}

def get_random_elements():
    elements = []
    for x in range(-16, 11):
        for y in range(-11, 11):
            center = [x + 0.9 * random(), 0.2 - (0.0 if x > -11 else 0.05), y + 0.9 * random()]
            if not center_ok(center): continue

            mat_func = choice([random_lambertian, random_glass, random_metal])
            material = mat_func()
            elements.append({"type": "sphere", "center": center, "radius": 0.2, "material": material})

            if material["type"] == "glass" and random() < 0.5:
                elements.append({"type": "sphere", "center": center, "radius": -0.19, "material": material})

    # big balls and ground
    b1 = {"type": "sphere", "center": [4.0, 1.0, 0.0], "radius": 1.0, "material": {"type": "glass", "color": [240, 240, 240], "refraction_index": 1.5}}
    b1_1 = {"type": "sphere", "center": [4.0, 1.0, 0.0], "radius": -0.95, "material": {"type": "glass", "color": [240, 240, 240], "refraction_index": 1.5}}
    b2 = {"type": "sphere", "center": [-4.0, 1.0, 0.0], "radius": 1.0, "material": {"type": "metal", "color": [252, 185, 15], "fuzziness": 0.2}}
    b3 = {"type": "sphere", "center": [0.0, 1.0, 0.0], "radius": 1.0, "material": {"type": "metal", "color": [179, 153, 128], "fuzziness": 0.03}}
    gnd = {"type": "sphere", "center": [0.0, -1000.0, 0.0], "radius": 1000.0, "material": {"type": "lambertian", "color": [128, 128, 128]}}

    elements += [b1, b1_1, b2, b3, gnd]
    return elements


def main():
    out_path = sys.argv[1]

    scene = {
        "img_width": 1920,
        "img_height": 1080,
        "title": "big_scene",
        "antialias_amount": 200,
        "max_bounces": 100,
        "out_format": "png",
        "background_top_color": [128, 178, 255],
        "background_bottom_color": [255, 255, 255],
        "camera": {
            "position": [13.0, 2.0, 3.0],
            "looking_at": [0.0, 0.5, 0.0],
            "up_vec": [0.0, 1.0, 0.0],
            "fov": 20.0,
            "aperture": 0.1,
            "focus_distance": 13.6
        },
        "elements": get_random_elements()
    }

    data = {"scenes": [scene]}
    with open(out_path, "w") as f:
        f.write(json.dumps(data))

main()