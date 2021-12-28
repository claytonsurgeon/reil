[Interesting Convo about Zig](https://news.ycombinator.com/item?id=29702607)

jstimpfle [Can it be this simple?](https://news.ycombinator.com/item?id=29706465)
It's more like, "I don't see why a tastier dinner has to be more expensive". It's a tradeoff situation, and while I might be willing to buy a great $50+ meal from time to time instead of just a $20 one, I don't have any inclination to pay $1000 even if the meal even a tiny little bit better than the just-great one.
It's a matter of tradeoffs. Library design is a balancing act.

> We need generics because users have types that they need libraries to work with.

This is right where I become sceptical. Most libraries shouldn't care for the user's types at all. They should expose their own types so you can work the library - not the other way around.

Please provide an actual use case where the library has to "know" about the user's types (and please don't mention std::sort. It's as common an example as is "Dog :: Animal" examples to argue for class inheritance, and is just as irrelevant).

> Your own example, of an intrusively-linked list, illustrates this: without generics, you could not write a linked-list library component that would be usable for that.

As I mentioned, linked list can tangentially benefit from a very very thing generics layer on top of a fixed data structure implementation. The layer does nothing more than "instanciate" the types, but there is no code generated. But then again, the added convenience / safety is minimal, I once wrote a C++ implementation that I was quite happy with, and never used it.

> This is why C programs are crammed with so many custom one-off hash tables

If you want to reuse a generic table, this is almost an upper bound of the work you should have to do even without generics.
~~~
int foo_key_equal(void *item, void *data) {
    Foo *foo = item;
    Fookey *key = data;
    return foo->key == key;
}    

Foo *lookup_foo(Hashtable *ht, Fookey key) {
    return ht_lookup(ht, hash_fookey(&key), &foo_key_equal, &key);
}

void insert_foo(Hashtable *ht, Foo *foo) {
    return ht_insert(ht, hash_fookey(&foo->key), foo);
}

void erase_foo(Hashtable *ht, Foo *foo) {
    return ht_erase(ht, hash_fookey(&foo->key), &foo_key_equal, &key);
}
~~~
It's not nothing of course, but as long as you're not doing scripting work where you're mapping tons of strings to things (in which case you should be using something like Python probably), it's not too bad. I will definitely think twice before including something like https://raw.githubusercontent.com/gcc-mirror/gcc/master/libs... instead. I'm probably willing to pay price for a little code like above given that I've had 1 use case for a hash map in 2021 (a redesign of a glyph hash table) that I've not even implemented - I've half-assed it with a simple for-loop that has never shown up in a performance profile.
An atlernative reason why you'll find a good amount of custom hashtables in C code bases is, I suppose, that there are a lot of different ways to implement hash tables.