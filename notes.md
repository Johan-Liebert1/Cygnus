*a + b

var a -> times deref = 1

binary exp *a + b -> times deref 0

---

*(((a))) + b

var a -> times deref = 1

binary exp *a + b -> times deref 0

--- 
*(a + b)

var a -> times deref = 0
var b -> times deref = 0

binary exp a + b -> times deref 1

--- 
*(a + *b)

var a -> times deref = 0
var b -> times deref = 1

binary exp a + *b -> times deref 1


1. Things in brackets should be evaluated first, then the whole deref thing should be applied

# Handling strings

def hi: str = "hello"

stack top
1008 5
1016 ptr to "hello"

---

def hi: *char = C"hello"

stack top
1008 ptr to "hello"

---

def hi: str = "hello"
def addr: *str = &hi;

stack top
1000 1016
1008 5
1016 ptr to "hello"

---
string deref is allowed

def hi: str = "hello"

write(*hi) -> 'h'
