## What's statically typed data language and what for?

 Thare are three goals I want to achieve by the Dochy language.
 
1. Efficiency
2. Version Awareness
3. Improved Expressiveness By Supporting Reference and Enum


# 1. Efficiency

 Dochy language is basically JSON5 which is a better JSON. JSON5 is dynamically typed, like this.
 
```json5
{
  list : [
	{
		age: 2,
		name: "dochy1"
	},
	{
		age: 3,
		name: "dochy2"
	}
  ]
}
```

Dynamically typed data must contain type data, namely member names and member types,
which can vary in each item so every item must have them.

In Dochy, member names and types are statically defined and can't vary in a list.
```json5
{
  // Comments can be written in JSON5 (and Dochy)
  list : [ 
    // "MList" means a mutable list, one of the collection types in Dochy.
    // Every collection must have the collection type as the first item
    "MList",
    // Every collection in Dochy must have Default Object written within "[{" and "}]".
    // Default Object defines the static type of the items in the list.
    [{
      age : 0, //The type is Int, which is distinguished from Float
      name : "" //String
    }],
    {
      age : 2,
      name : "dochy1"
    },
    {
      age : 3,
      name : "dochy2",
    }
  ]
}
```
Static types can't vary, so we can omit the type data. 