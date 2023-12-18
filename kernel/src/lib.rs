pub struct Vertex<T>(pub T);

pub struct Line<T> {
    from: Vertex<T>,
    to: Vertex<T>,
}

pub struct Plane<T>(pub T, pub T, pub T);

pub struct Sphere<T, U> {
    center: T,
    radius: U,
}
