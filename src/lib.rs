use amethyst_core::ecs as specs;

#[derive(Default)]
pub struct TestResource(pub i32);
#[derive(Default)]
pub struct TestResourceTwo(pub i32);
#[derive(Default)]
pub struct TestResourceThree(pub i32);
#[derive(Default)]
pub struct TestResourceFour(pub i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TestCompOne(pub f32, pub f32, pub f32);
impl specs::Component for TestCompOne {
    type Storage = specs::DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TestCompTwo(pub f32, pub f32, pub f32);
impl specs::Component for TestCompTwo {
    type Storage = specs::DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TestCompThree(pub f32, pub f32, pub f32);
impl specs::Component for TestCompThree {
    type Storage = specs::DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TestCompFour(pub f32, pub f32, pub f32);
impl specs::Component for TestCompFour {
    type Storage = specs::DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TestCompFive(pub f32, pub f32, pub f32);
impl specs::Component for TestCompFive {
    type Storage = specs::DenseVecStorage<Self>;
}
