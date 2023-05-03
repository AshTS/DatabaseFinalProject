import csv

classes = {"Bard": "Bard", "Cleric": "Cleric", "Druid": "Druid", "Paladin": "Paladin", "Ranger": "Ranger", "Sorcerer/Wizard": "Wizard", "Sorcerer": "Wizard", "Wizard": "Wizard"}

def first_number(s):
    for i in range(len(s)):
        if s[i].isdigit():
            length = 1
            while i + length < len(s) and s[i + length].isdigit():
                length += 1
            return int(s[i:i + length])
    return None

reader = csv.reader(open("original.csv", "r").readlines())
output = open("out.csv", "w")
output = csv.writer(output)

class_output = open("class-pairs.csv", "w")
class_output = csv.writer(class_output)



header = next(reader)
out_headers = ["spell_id", "name", "range", "duration", "somatic", "verbal", "material", "area", "other"]
output.writerow(out_headers)

class_output.writerow(["spell_id", "class", "level", "other"])

SPELL_ID = 0

for row in reader:
    d = {key: value for (key, value) in zip(header, row)}

    skip = False

    range_distance = first_number(d["range"])
    if range_distance is None:
        if "touch" in d["range"].lower() or d["range"] == "None":
            range_distance = 0
        elif "personal" in d["range"].lower():
            range_distance = 5
        elif d["range"] == "One mile":
            range_distance = 5280
        elif d["range"] == "Unlimited":
            range_distance = 2**32 - 1
        else:
            continue

    duration = None
    if "instantaneous" in d["duration"].lower():
        duration = 0
    elif "min" in d["duration"].lower():
        duration = first_number(d["duration"]) * 60
    elif "hour" in d["duration"].lower():
        duration = first_number(d["duration"]) * 3600
    elif "round" in d["duration"].lower():
        duration = first_number(d["duration"]) * 6
    elif "one day" in d["duration"].lower():
        duration = 24 * 3600
    else:
        continue

    verbal = "v" in d["components"].lower()
    somatic = "s" in d["components"].lower()
    material = "m" in d["components"].lower()

    area = not "None" in d["area"]

    level_pairs = []

    if d["level"] != "None":
        entries = d["level"].split(", ")
        for entry in entries:
            if len(entry.split(" ")) == 2:
                name, number = entry.split(" ")
                if name in classes:
                    level_pairs.append((classes[name], int(number)))
                    pass
                else:
                    pass
            else:
                pass

        print(level_pairs)

        for name, level in level_pairs:
            class_output.writerow([SPELL_ID, name, level, "other"])

    row = [SPELL_ID, d["name"], range_distance, duration, somatic, verbal, material, area, "Other"]
    output.writerow(row)

    SPELL_ID += 1

    