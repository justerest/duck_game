# ...

```mermaid
classDiagram

class Duck {
	+velocity: XY~Velocity~
	+pos() Vec2
}

class World {
	-solids: unknown
	+duck_position: Vec2
	+move_duck(duck: &Duck)
}
```