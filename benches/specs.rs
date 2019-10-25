use amethyst_core::{ecs::prelude::*, SystemDesc, Transform};
use amethyst_ecs_benchmarks::*;
use criterion::*;
use rand::Rng;

fn create_transforms(c: &mut Criterion) {
    let prepare = || {
        let mut world = World::new();
        world.register::<Transform>();
        world
    };
    let run1000 = |mut world: World| {
        for _ in 0..=1000 {
            world.create_entity().with(Transform::default()).build();
        }
    };
    let run10000 = |mut world: World| {
        for _ in 0..=10000 {
            world.create_entity().with(Transform::default()).build();
        }
    };

    c.bench_function("specs create_transforms 1000", move |b| {
        b.iter_batched(prepare, run1000, BatchSize::SmallInput);
    });
    c.bench_function("specs create_transforms 10000", move |b| {
        b.iter_batched(prepare, run10000, BatchSize::SmallInput);
    });
}

fn moving_objects(c: &mut Criterion) {
    extern crate criterion;
    use criterion::{BatchSize, Criterion};

    use amethyst_core::transform::*;
    use specs_hierarchy::HierarchySystem;
    let setup = || {
        // Instantiate World
        let mut world = World::new();

        {
            // Create entities
            let ents: Vec<Entity> = world.create_iter().take(1000).collect();
            let mut transforms = world.write_storage::<Transform>();

            for (i, e) in ents.iter().enumerate() {
                transforms.insert(*e, Transform::default()).unwrap();
            }
        }

        // Build Dispatcher
        let mut builder = DispatcherBuilder::new();
        builder.add(
            HierarchySystem::<Parent>::new(&mut world),
            "hierarchy_system",
            &[],
        );
        builder.add(
            TransformSystemDesc::default().build(&mut world),
            "transform_system",
            &["hierarchy_system"],
        );
        let mut dispatcher = builder.build();
        dispatcher.setup(&mut world);

        // Return world and dispatcher
        (world, dispatcher)
    };
    /*
    c.bench_function("specs transform_test", move |b| {
        b.iter_batched(
            setup,
            |(mut world, mut dispatcher)| dispatcher.dispatch(&mut world),
            BatchSize::SmallInput,
        );
    });*/
}

mod add_remove_components {
    use super::*;

    #[derive(Default)]
    pub struct TestSystem;
    impl<'a> System<'a> for TestSystem {
        #[allow(clippy::type_complexity)]
        type SystemData = (
            Entities<'a>,
            WriteStorage<'a, TestCompOne>,
            WriteStorage<'a, TestCompTwo>,
            WriteStorage<'a, TestCompThree>,
            WriteStorage<'a, TestCompFour>,
            WriteStorage<'a, TestCompFive>,
        );

        fn run(
            &mut self,
            (entities, mut one, mut two, mut three, mut four, mut five): Self::SystemData,
        ) {
            {
                let remove = (&entities, &mut one)
                    .join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let insert = (&entities, &mut one)
                    .join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                remove.iter().for_each(|e| {
                    one.remove(*e);
                });

                insert.iter().for_each(|e| {
                    one.insert(*e, TestCompOne(1., 2., 3.)).unwrap();
                });
            }

            {
                let remove = (&entities, &mut two)
                    .join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let insert = (&entities, &mut two)
                    .join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                remove.iter().for_each(|e| {
                    two.remove(*e);
                });

                insert.iter().for_each(|e| {
                    two.insert(*e, TestCompTwo(1., 2., 3.)).unwrap();
                });
            }

            {
                let remove = (&entities, &mut three)
                    .join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let insert = (&entities, &mut three)
                    .join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                remove.iter().for_each(|e| {
                    three.remove(*e);
                });

                insert.iter().for_each(|e| {
                    three.insert(*e, TestCompThree(1., 2., 3.)).unwrap();
                });
            }
        }
    }

    pub fn bench(c: &mut Criterion) {
        let prepare = |entity_count: usize| {
            // Instantiate World
            let mut world = World::new();

            world.register::<TestCompOne>();
            world.register::<TestCompTwo>();
            world.register::<TestCompThree>();
            world.register::<TestCompFour>();
            world.register::<TestCompFive>();

            {
                // Create entities
                let entities: Vec<Entity> = world.create_iter().take(entity_count).collect();
                let (mut one, mut two, mut three) = <(
                    WriteStorage<'_, TestCompThree>,
                    WriteStorage<'_, TestCompFour>,
                    WriteStorage<'_, TestCompFive>,
                )>::fetch(&mut world);

                entities.iter().for_each(|e| {
                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                        one.insert(*e, TestCompThree(1., 2., 3.)).unwrap();
                    }
                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                        two.insert(*e, TestCompFour(1., 2., 3.)).unwrap();
                    }
                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                        three.insert(*e, TestCompFive(1., 2., 3.)).unwrap();
                    }
                });
            }

            // Build Dispatcher
            let mut builder = DispatcherBuilder::new();
            builder.add(TestSystem::default(), "test_system", &[]);

            let mut dispatcher = builder.build();
            dispatcher.setup(&mut world);

            // Return world and dispatcher
            (world, dispatcher)
        };

        c.bench_function("specs add_remove_components 1000", move |b| {
            b.iter_batched(
                || prepare(1000),
                |(mut world, mut dispatcher)| {
                    dispatcher.dispatch(&mut world);
                },
                BatchSize::SmallInput,
            );
        });

        c.bench_function("specs add_remove_components 10000", move |b| {
            b.iter_batched(
                || prepare(10000),
                |(mut world, mut dispatcher)| {
                    dispatcher.dispatch(&mut world);
                },
                BatchSize::SmallInput,
            );
        });
    }
}

mod par_add_remove_components {
    use super::*;

    #[derive(Default)]
    pub struct TestSystem;
    impl<'a> System<'a> for TestSystem {
        #[allow(clippy::type_complexity)]
        type SystemData = (
            Entities<'a>,
            WriteStorage<'a, TestCompOne>,
            WriteStorage<'a, TestCompTwo>,
            WriteStorage<'a, TestCompThree>,
            WriteStorage<'a, TestCompFour>,
            WriteStorage<'a, TestCompFive>,
        );

        fn run(
            &mut self,
            (entities, mut one, mut two, mut three, mut four, mut five): Self::SystemData,
        ) {
            {
                let remove = (&entities, &mut one)
                    .par_join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let insert = (&entities, &mut one)
                    .par_join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                remove.iter().for_each(|e| {
                    one.remove(*e);
                });

                insert.iter().for_each(|e| {
                    one.insert(*e, TestCompOne(1., 2., 3.)).unwrap();
                });
            }

            {
                let remove = (&entities, &mut two)
                    .par_join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let insert = (&entities, &mut two)
                    .par_join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                remove.iter().for_each(|e| {
                    two.remove(*e);
                });

                insert.iter().for_each(|e| {
                    two.insert(*e, TestCompTwo(1., 2., 3.)).unwrap();
                });
            }

            {
                let remove = (&entities, &mut three)
                    .par_join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let insert = (&entities, &mut three)
                    .par_join()
                    .filter_map(|(e, _)| {
                        if rand::thread_rng().gen_range(0, 1000) > 500 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                remove.iter().for_each(|e| {
                    three.remove(*e);
                });

                insert.iter().for_each(|e| {
                    three.insert(*e, TestCompThree(1., 2., 3.)).unwrap();
                });
            }
        }
    }

    pub fn bench(c: &mut Criterion) {
        let prepare = |entity_count: usize| {
            // Instantiate World
            let mut world = World::new();

            world.register::<TestCompOne>();
            world.register::<TestCompTwo>();
            world.register::<TestCompThree>();
            world.register::<TestCompFour>();
            world.register::<TestCompFive>();

            {
                // Create entities
                let entities: Vec<Entity> = world.create_iter().take(entity_count).collect();
                let (mut one, mut two, mut three) = <(
                    WriteStorage<'_, TestCompThree>,
                    WriteStorage<'_, TestCompFour>,
                    WriteStorage<'_, TestCompFive>,
                )>::fetch(&mut world);

                entities.iter().for_each(|e| {
                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                        one.insert(*e, TestCompThree(1., 2., 3.)).unwrap();
                    }
                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                        two.insert(*e, TestCompFour(1., 2., 3.)).unwrap();
                    }
                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                        three.insert(*e, TestCompFive(1., 2., 3.)).unwrap();
                    }
                });
            }

            // Build Dispatcher
            let mut builder = DispatcherBuilder::new();
            builder.add(TestSystem::default(), "test_system", &[]);

            let mut dispatcher = builder.build();
            dispatcher.setup(&mut world);

            // Return world and dispatcher
            (world, dispatcher)
        };

        c.bench_function("specs par_add_remove_components 1000", move |b| {
            b.iter_batched(
                || prepare(1000),
                |(mut world, mut dispatcher)| {
                    dispatcher.dispatch(&mut world);
                },
                BatchSize::SmallInput,
            );
        });

        c.bench_function("specs par_add_remove_components 10000", move |b| {
            b.iter_batched(
                || prepare(10000),
                |(mut world, mut dispatcher)| {
                    dispatcher.dispatch(&mut world);
                },
                BatchSize::SmallInput,
            );
        });
    }
}

criterion_group!(
    benches,
    create_transforms,
    add_remove_components::bench,
    par_add_remove_components::bench,
    moving_objects
);
criterion_main!(benches);
