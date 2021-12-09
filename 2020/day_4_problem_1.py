"""
--- Day 4: Passport Processing ---

You arrive at the airport only to realize that you grabbed your North Pole
Credentials instead of your passport. While these documents are extremely
similar, North Pole Credentials aren't issued by a country and therefore aren't
actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though; a very long line
has formed for the automatic passport scanners, and the delay could upset your
travel itinerary.

Due to some questionable network security, you realize you might be able to
solve both of these problems at the same time.

The automatic passport scanners are slow because they're having trouble
detecting which passports have all required fields. The expected fields are as
follows:

byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)

Passport data is validated in batch files (your puzzle input). Each passport is
represented as a sequence of key:value pairs separated by spaces or newlines.
Passports are separated by blank lines.

Here is an example batch file containing four passports:

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

The first passport is valid - all eight fields are present. The second passport
is invalid - it is missing hgt (the Height field).

The third passport is interesting; the only missing field is cid, so it looks
like data from North Pole Credentials, not a passport at all! Surely, nobody
would mind if you made the system temporarily ignore missing cid fields. Treat
this "passport" as valid.

The fourth passport is missing two fields, cid and byr. Missing cid is fine,
but missing any other field is not, so this passport is invalid.

According to the above rules, your improved system would report 2 valid
passports.

Count the number of valid passports - those that have all required fields.
Treat cid as optional. In your batch file, how many passports are valid?

"""

demo_input = \
"""ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"""
demo_input = demo_input.split("\n")


file_name = "day_4_input.txt"
real_input = []
with open(file_name) as f:
    file_input = f.readlines()
for line in file_input:
    real_input.append(line[:-1])
# this is a bit hacky
real_input.append("")

pass_scan = real_input
# for testing
#pass_scan = demo_input

necessary_codes = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
passport_list = []
passport = []

for line in pass_scan:
    print(f"line: {line}")
    items = line.split(" ")
    for item in items:
        key_val = item.split(":")
        #print(f"item_{key_val}")
        if key_val[0] == "":
            #passport = sorted(passport, key=str)
            passport_list.append(passport)
            print(f"passport: {passport}\n")
            passport = []
           # print("new passport")
        else:
            passport.append(key_val[0])


# items = set([-1, 0, 1, 2])
# set([1, 2]).issubset(items)
# True
# set([1, 3]).issubset(items)
# False


valid_passports = 0
invalid_passports = 0

code_set = set(necessary_codes)

for passport in passport_list:

    pass_set = set(passport)

# Set A is said to be the subset of set B
# if all elements of A are in B.
# A.issubset(B)
# The above code checks if A is a subset of B.
# if all code_set.issubset(B) is in B

    if code_set.issubset(pass_set):
        print(f"valid:   {passport}")
        valid_passports += 1
    else:
        print(f"INVALID: {passport}")
        invalid_passports += 1

print(f"{code_set}")
print(f"Found {valid_passports} valid_passports")
print(f"{invalid_passports} invalid passports")
print(f"{valid_passports+invalid_passports} total")


