# About

My second prototype of an inventory-oriented crate for Bevy game engine.

This prototype contains:
- Item entities which can inherit properties of their prototype-like parent item entities,
- Component-based metadata, most controversial being "item count" in an inventory,
- Inventory entities which contain a constant size array of item slots (each slot can contain an item),
- Crude UI
- Picking up and dropping items between slots by spawning a new 1-slot inventory
- Incomplete drop item on another item behavior



# Issues

While developing this prototype there were many adressed and unadressed issues.


## Inventory models

There are many inventory models, this prototype attempts to solve a lot of them, but not all of them.
Specificaly, this crate allows for one-slot-per-item systems.
Inventory-management-oriented games often use multiple-slots-per-item mechanic, which requires a vastly different setup.

That said, item inheritance logic can be applied to all inventory systems and things like "shape" could be inheritable.
Perhaps it should be a separate crate once perfected.


## Item dependency traversal

Can be expensive and a copy-all-components approach should be considered.
Items are (in most cases) defined during loading, there is no point to do the parent lookup at game runtime.

Few possible solutions:
- Copy parent during creation, provide API for appropriate ordering, no parent mutations after that,
- Use a reactive approach, which lazily copies parent components to their children, only on changes,
- Provide API (probably a command) for flushing the dependency tree, which will copy all parent components to their children.


## Slot-item relations

While slots point to (optinally) a single item, items have no limit in how many slots they can be in.
This has few issues:
- If items store their own count in the inventory, 2 slots containing the same item means they share the same count (this could be intentional),
- Items meant to be prototypes can also be in the slots.


## UI templates

All games have their own stylized UI.
Games will also add their own mechanics which will add or subtract from the UI.
This requires a really flexible UI template system, which ensures the inventory-slot-item hierarchy, while still giving developers full freedom.

A partial solution would be to use something like [core/headless widgets](https://github.com/viridia/thorium_ui/tree/main/crates/thorium_ui_headless), but this doesn't address the required hierarchy issues.


## Other

- Support prediction for networking,
- Buffering inventory operations (using observers was lazy of me),
- Fallible operations and generally edge cases (like cursor being removed when we close the inventory, what happens to contained items?).
