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

--- Part Two ---

The line is moving more quickly now, but you overhear airport security talking
about how passports with invalid data are getting through. Better add some data
validation, quick!

You can continue to ignore the cid field, but each other field has strict rules
about what values are valid for automatic validation:

byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
If cm, the number must be at least 150 and at most 193.
If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present
and valid according to the above rules. Here are some example values:

byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789
Here are some invalid passports:

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
Here are some valid passports:

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

Count the number of valid passports - those that have all required fields and
valid values. Continue to treat cid as optional. In your batch file, how many
passports are valid?

"""
class Passport:

    def __init__(self):
        self.byr = 0 # (Birth Year) four digits 1920-2002
        self.iyr = 0 # (Issue Year) four digits 2010-2020
        self.eyr = 0 # (Expiration Year) four digits; 2020-2030
        self.hgt = 0 # (Height) a number followed by either cm or in
        self.hgt_unit = "?"
            # If cm, the number must be at least 150 and at most 193.
            # If in, the number must be at least 59 and at most 76.
        self.hcl = "?" # (Hair Color) '#' followed by exactly six characters 0-9 or a-f.        
        self.ecl = "?" # (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        self.pid = "?" # (Passport ID) - a nine-digit number, including leading zeroes.
        self.cid = "?" # ignored

        self.byr_valid = ""
        self.iyr_valid = ""
        self.eyr_valid = ""
        self.hgt_valid = "" 
        self.hgt_unit_valid = ""
        self.hcl_valid = ""
        self.ecl_valid = ""
        self.pid_valid = ""
        self.cid_valid = ""


    def print_info(self):
        print(f"byr {self.byr} {self.byr_valid}")
        print(f"iyr {self.iyr} {self.iyr_valid}")
        print(f"eyr {self.eyr} {self.eyr_valid}")
        print(f"hgt {self.hgt} {self.hgt_valid}")
        print(f"hgt_unit {self.hgt_unit} {self.hgt_unit_valid}")
        print(f"hcl {self.hcl} {self.hcl_valid}")
        print(f"ecl {self.ecl} {self.ecl_valid}")
        print(f"pid {self.pid} {self.pid_valid}")
       # print(f"cid {self.cid} {self.cid_valid}")

    def is_valid(self):
        self._check_validity()

        invalid_vars = [self.byr_valid, self.iyr_valid, self.eyr_valid,\
                self.hgt_valid, self.hgt_unit_valid, self.hcl_valid,\
                self.ecl_valid, self.pid_valid, self.cid_valid] 
        for valid in invalid_vars:
            if len(valid) > 0:
                return False
        return True


    # what about missing values 137 was to high for valid
    def _check_validity(self):
        # look for invalid conditions and on the first one return false
        self.byr_valid = ""
        self.iyr_valid = ""
        self.eyr_valid = ""
        self.hgt_valid = "" 
        self.hgt_unit_valid = ""
        self.hcl_valid = ""
        self.ecl_valid = ""
        self.pid_valid = ""
        self.cid_valid = ""
        
        # (Birth Year) four digits 1920-2002
        if self.byr > 2002:
            self.byr_valid = f"invalid: self.byr > 2002"
        if self.byr < 1920:
            self.byr_valid = f"invalid: self.byr < 1920"
        
        # (Issue Year) four digits 2010-2020 
        if self.iyr > 2020:
            self.iyr_valid = f"invalid: self.iyr > 2020"
        if self.iyr < 2010:
            self.iyr_valid = f"invalid: self.byr < 2010"
        
        # (Expiration Year) four digits; 2020-2030 
        if self.eyr > 2030:
            self.eyr_valid = f"invalid: self.eyr > 2030"
        if self.eyr < 2020:
            self.eyr_valid = f"invalid: self.byr < 2020"
        
        # (Height) a number followed by either cm or in
         # If cm, the number must be at least 150 and at most 193.
         # If in, the number must be at least 59 and at most 76.
        if self.hgt_unit == "cm":
            if self.hgt < 150:
                self.hgt_valid = f"invalid: self.hgt < 150"
            if self.hgt > 193:
                self.hgt_valid = f"invalid: self.hgt > 193"
        elif self.hgt_unit == "in":
            if self.hgt > 76:
                self.hgt_valid = f"invalid: self.hgt > 76"
            if self.hgt < 59:
                self.hgt_valid = f"invalid: self.hgt < 59"
        else:
            self.hgt_unit_valid = f"invalid: self.hgt_unit unknown"
            self.hgt_valid = "idk"

        # (Hair Color) '#' followed by exactly six characters 0-9 or a-f.        
        if self.hcl[0] == "#":
            valid_chars = set(list("0123456789abcdef")) 
            chars = set(self.hcl[1:])
            # if all elements of A are in B.
            # A.issubset(B)
            if not chars.issubset(valid_chars):
                self.hcl_valid = f"invalid: self.hcl invalid chars"
        else:
            self.hcl_valid = f"unknown (no #)"

        # (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if len(self.ecl) == 3:
            valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            found = False
            for color in valid_colors:
                if self.ecl == color:
                    found = True
                    break
            if not found:
                self.ecl_valid = f"invalid: self.ecl invalid colors"
        
        # (Passport ID) - a nine-digit number, including leading zeroes.
        if len(self.pid) == 9:
            valid_chars = set(list("0123456789")) 
            pid_chars = set(list(self.pid))
            if not pid_chars.issubset(valid_chars):
                self.pid_valid = f"invalid: self.pid invalid chars"
        else:
            self.pid_valid = f"invalid: self.pid invalid length"

 
    def process_key_val(self, key_val):
        """
        one line from input file
        get some basic checks and store some stuff so we can check later
        put in right datatype
        """
        if key_val[0] != "":
            key = key_val[0]
            val = key_val[1]
            if key == "byr":
                # 4 digits
                if len(val) == 4:
                    self.byr = int(val)
            if key == "iyr":
                # 4 digits
                if len(val) == 4:
                    self.iyr = int(val)
            if key == "eyr":
                # 4 digits
                if len(val) == 4:
                    self.eyr = int(val)
            if key == "hgt":
                if val[-2:] == "cm":
                    self.hgt_unit = "cm"
                elif val[-2:] == "in":
                    self.hgt_unit = "in"
                if len(val) > 2:
                    self.hgt = int(val[:-2])
            if key == "hcl":
                # '#' followed by exactly six characters 0-9 or a-f
                if len(val) == 7:
                    self.hcl = val
            if key == "ecl":
                self.ecl = val
            if key == "pid":
                if len(val) == 9:
                    self.pid = val
            



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
passport = Passport()

for line in pass_scan:
    
    if len(line) > 1:

        items = line.split(" ")
        for item in items:
            key_val = item.split(":")
            if len(key_val[0]) > 0:
                passport.process_key_val(key_val)
    else:
        if not passport.is_valid():
            print("")
            passport.print_info()
        passport_list.append(passport)
        passport = Passport()

# items = set([-1, 0, 1, 2])
# set([1, 2]).issubset(items)
# True
# set([1, 3]).issubset(items)
# False

valid_count = 0
invalid_count = 0

for passport in passport_list:
    if passport.is_valid(): 
        #print(f"valid:   {passport}")
        valid_count += 1
    else:
        invalid_count += 1

print(f"Found\n!{valid_count} valid passports!")
print(f"{invalid_count} invalid passports")
print(f"{valid_count+invalid_count} total")

# 137 valid
# 155 invalid
# 292 total










