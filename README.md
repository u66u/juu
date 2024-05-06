Proposed syntax:

```Haskell
type Animal:
  age: Int
  name: String
  paws: Int
  
impl Animal:
  fn new(age: @Int, name: @String, paws: @Int) -> Self:
    return Self(age: age, name: name, paws: paws)
  
  fn birthday(animal: !@This, olderby: @Int) -> Self:
    return animal.age += olderby
    
 trait Barks:
  fn bark(animal: @This) -> String:
    print("Bark!")
    
 impl Barks for Animal:
  fn bark(animal: @This) -> String:
    print("bark bark!")
    
def main():
  let mut dog = Animal(age: 5, name: "susan", paws: 4)
  dog.bark() -- prints "bark bark!"
  dog.birthday(2) -- now age is 7
  
  let cat = Animal.new()
  cat.birthday(1) -- Error
```
