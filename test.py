my = {97: 47,
98: 4,
99: 26,
100: 25,
101: 67,
102: 15,
103: 2,
104: 4,
105: 66,
106: 0,
107: 0,
108: 29,
109: 22,
110: 28,
111: 44,
112: 26,
113: 2,
114: 39,
115: 31,
116: 54,
117: 40,
118: 4,
119: 0,
120: 9,
121: 0,
122: 0}

python = {}

file = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis."

for char in file:
    if ord(char) >= ord('a') and ord(char) <= ord('z'):
        if ord(char) in python:
            python[ord(char)] += 1
        else:
            python[ord(char)] = 1


for (k, v) in python.items():
    if my[k] != v:
        print(f"{k} is not equal")