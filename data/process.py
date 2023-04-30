import csv

def first_number(s):
    for i in range(len(s)):
        if s[i].isdigit():
            length = 1
            while i + length < len(s) and s[i + length].isdigit():
                length += 1
            return int(s[i:i + length])
    return None

output = open("out.csv", "w")
reader = csv.reader(open("original.csv", "r").readlines())
output = csv.writer(output)

header = next(reader)
out_headers = ["name", "range", "duration", "somatic", "verbal", "material", "other"]
output.writerow(out_headers)

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

    row = [d["name"], range_distance, duration, somatic, verbal, material, "Other"]
    output.writerow(row)

    