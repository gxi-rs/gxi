# Context Trees

The Context Tree data-structure is a unique way of handling `state`,
`view-nodes`, and nested `context trees` in an observable event based
view node system.

The tree structure revolves around reference counted smart pointers,
which in rust are `std::rc::Rc` and `std::rc::Weak`. In-order to prevent
reference cycles and memory leak, the tree needs to be used in a specific
principal layout which should be adhered to throughout the program.

This pattern aims to bring a scalable and less overhead solution as
apposed to the standard practice of `virtual-dom` and `diffing` algorithms.

As a rule of thumb, only those nodes should remain in memory which are required
by the state pattern of a component. A context node can own arbitrary amount of
memory inside itself for the lifetime of a component. The lifetime of this node
depends upon the lifetime of state owners. 


## The tree structure


```md
   { (componet scope)
      
      [state]
         | 
         |
         |  [observable node]
         |        /
         |        | 
          -   (view)
   
   }
```



## Observable Pattern

The observable system is nothing more than a wrapper around, vectors of
observable callback closures. The list of `observers` can be notified about
a state change using an associated `notify()` function.

`&RefCell<V>` is passed to the `observer` to prevent multiple mutable borrows.

## 
