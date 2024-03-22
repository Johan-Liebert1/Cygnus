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
