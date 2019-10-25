use amethyst_core::legion::{transform::components::*, *};
use amethyst_ecs_benchmarks::*;
use criterion::*;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;

fn bench_create_transforms(c: &mut Criterion) {
    let prepare = || {
        let universe = Universe::new();
        universe.create_world()
    };
    let run1000 = |mut world: World| {
        world.insert((), (0..=1000).map(|_| (LocalToWorld::default(),)));
    };
    let run10000 = |mut world: World| {
        world.insert((), (0..=10000).map(|_| (LocalToWorld::default(),)));
    };

    c.bench_function("legion create_transforms 1000", move |b| {
        b.iter_batched(prepare, run1000, BatchSize::SmallInput);
    });
    c.bench_function("legion create_transforms 10000", move |b| {
        b.iter_batched(prepare, run10000, BatchSize::SmallInput);
    });
}

fn legion_transform_system(c: &mut Criterion) {
    let prepare = |entity_count: usize| {
        let universe = Universe::new();
        let mut world = universe.create_world();

        // Insert n entities with base transforms
        world.insert(
            (),
            (0..=entity_count).map(|n| {
                (
                    Translation::new(n as f32, n as f32, n as f32),
                    LocalToWorld::default(),
                )
            }),
        );

        // Create a legion dispatcher
        let mut builder = DispatcherBuilder::default();

        // Add the transform bundle
        transform::TransformBundle::default().build(&mut world, &mut builder);

        // Build the dispatcher and return our setup
        let dispatcher = builder.build(&mut world).finalize();
        (world, dispatcher)
    };

    c.bench_function("legion add_remove_components 1000", move |b| {
        b.iter_batched(
            || prepare(1000),
            |(world, dispatcher)| {},
            BatchSize::SmallInput,
        );
    });
}

fn bench_add_remove_components(c: &mut Criterion) {
    let prepare = |entity_count: usize, defrag: Option<usize>| {
        let universe = Universe::new();
        let mut world = universe.create_world();

        world.resources.insert(Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(8)
                .build()
                .unwrap(),
        ));

        // Insert n entities with base transforms
        let entities = world
            .insert((), (0..=entity_count).map(|n| (LocalToWorld::default(),)))
            .to_vec();

        // Randomly add comp 3/4/5 to everything to get different archetypes
        entities.iter().for_each(|e| {
            if rand::thread_rng().gen_range(0, 1000) > 500 {
                world.add_component(*e, TestCompThree(1., 2., 3.));
            }
            if rand::thread_rng().gen_range(0, 1000) > 500 {
                world.add_component(*e, TestCompFour(1., 2., 3.));
            }
            if rand::thread_rng().gen_range(0, 1000) > 500 {
                world.add_component(*e, TestCompFive(1., 2., 3.));
            }
        });

        // Create a legion dispatcher
        let mut builder = DispatcherBuilder::default()
            .with_defrag_budget(defrag)
            .with_system(Stage::Logic, |_| {
                SystemBuilder::<()>::new("legion add_remove_components")
                    .with_query(<(Read<LocalToWorld>, Read<TestCompOne>)>::query())
                    .with_query(<(Read<LocalToWorld>)>::query().filter(!component::<TestCompOne>()))
                    .build(
                        move |command_buffer, _, mut thread_rng, (query_with, query_without)| {
                            // if the component exists, *randomly* remove it
                            {
                                query_with.iter_entities().for_each(|(e, (_, _))| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.remove_component::<TestCompOne>(e);
                                    }
                                });

                                // if it doesnt exist, add it, *randomly* add it
                                query_without.iter_entities().for_each(|(e, _)| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.add_component(e, TestCompOne(1., 2., 3.));
                                    }
                                });
                            }
                            {
                                query_with.iter_entities().for_each(|(e, (_, _))| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.remove_component::<TestCompTwo>(e);
                                    }
                                });

                                // if it doesnt exist, add it, *randomly* add it
                                query_without.iter_entities().for_each(|(e, _)| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.add_component(e, TestCompTwo(1., 2., 3.));
                                    }
                                });
                            }
                            {
                                query_with.iter_entities().for_each(|(e, (_, _))| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.remove_component::<TestCompThree>(e);
                                    }
                                });

                                // if it doesnt exist, add it, *randomly* add it
                                query_without.iter_entities().for_each(|(e, _)| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.add_component(e, TestCompThree(1., 2., 3.));
                                    }
                                });
                            }
                        },
                    )
            });

        // Build the dispatcher and return our setup
        let dispatcher = builder.build(&mut world).finalize();
        (world, dispatcher)
    };

    c.bench_function("legion add_remove_components_defrag1000", move |b| {
        b.iter_batched(
            || prepare(1000, None),
            |(mut world, mut dispatcher)| {
                dispatcher.run(&mut world);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("legion add_remove_components_defrag 10000", move |b| {
        b.iter_batched(
            || prepare(10000, None),
            |(mut world, mut dispatcher)| {
                dispatcher.run(&mut world);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("legion add_remove_components_nodefrag 1000", move |b| {
        b.iter_batched(
            || prepare(1000, Some(0)),
            |(mut world, mut dispatcher)| {
                dispatcher.run(&mut world);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("legion add_remove_components_nodefrag 10000", move |b| {
        b.iter_batched(
            || prepare(10000, Some(0)),
            |(mut world, mut dispatcher)| {
                dispatcher.run(&mut world);
            },
            BatchSize::SmallInput,
        );
    });
}

fn par_bench_add_remove_components(c: &mut Criterion) {
    let prepare = |entity_count: usize, defrag: Option<usize>| {
        let universe = Universe::new();
        let mut world = universe.create_world();

        world.resources.insert(Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(8)
                .build()
                .unwrap(),
        ));

        // Insert n entities with base transforms
        let entities = world
            .insert((), (0..=entity_count).map(|n| (LocalToWorld::default(),)))
            .to_vec();

        // Randomly add comp 3/4/5 to everything to get different archetypes
        entities.iter().for_each(|e| {
            if rand::thread_rng().gen_range(0, 1000) > 500 {
                world.add_component(*e, TestCompThree(1., 2., 3.));
            }
            if rand::thread_rng().gen_range(0, 1000) > 500 {
                world.add_component(*e, TestCompFour(1., 2., 3.));
            }
            if rand::thread_rng().gen_range(0, 1000) > 500 {
                world.add_component(*e, TestCompFive(1., 2., 3.));
            }
        });

        // Create a legion dispatcher
        let mut builder = DispatcherBuilder::default()
            .with_defrag_budget(defrag)
            .with_system(Stage::Logic, |_| {
                SystemBuilder::<()>::new("legion add_remove_components")
                    .with_query(<(Read<LocalToWorld>, Read<TestCompOne>)>::query())
                    .with_query(<(Read<LocalToWorld>)>::query().filter(!component::<TestCompOne>()))
                    .build(
                        move |command_buffer, _, mut thread_rng, (query_with, query_without)| {
                            // if the component exists, *randomly* remove it
                            {
                                query_with.par_entities_for_each(|(e, (_, _))| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.remove_component::<TestCompOne>(e);
                                    }
                                });

                                // if it doesnt exist, add it, *randomly* add it
                                query_without.par_entities_for_each(|(e, _)| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.add_component(e, TestCompOne(1., 2., 3.));
                                    }
                                });
                            }
                            {
                                query_with.par_entities_for_each(|(e, (_, _))| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.remove_component::<TestCompTwo>(e);
                                    }
                                });

                                // if it doesnt exist, add it, *randomly* add it
                                query_without.par_entities_for_each(|(e, _)| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.add_component(e, TestCompTwo(1., 2., 3.));
                                    }
                                });
                            }
                            {
                                query_with.par_entities_for_each(|(e, (_, _))| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.remove_component::<TestCompThree>(e);
                                    }
                                });

                                // if it doesnt exist, add it, *randomly* add it
                                query_without.par_entities_for_each(|(e, _)| {
                                    if rand::thread_rng().gen_range(0, 1000) > 500 {
                                        command_buffer.add_component(e, TestCompThree(1., 2., 3.));
                                    }
                                });
                            }
                        },
                    )
            });

        // Build the dispatcher and return our setup
        let dispatcher = builder.build(&mut world).finalize();
        (world, dispatcher)
    };

    c.bench_function("legion par_add_remove_components_defrag1000", move |b| {
        b.iter_batched(
            || prepare(1000, None),
            |(mut world, mut dispatcher)| {
                dispatcher.run(&mut world);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("legion par_add_remove_components_defrag 10000", move |b| {
        b.iter_batched(
            || prepare(10000, None),
            |(mut world, mut dispatcher)| {
                dispatcher.run(&mut world);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("legion par_add_remove_components_nodefrag 1000", move |b| {
        b.iter_batched(
            || prepare(1000, Some(0)),
            |(mut world, mut dispatcher)| {
                dispatcher.run(&mut world);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function(
        "legion par_add_remove_components_nodefrag 10000",
        move |b| {
            b.iter_batched(
                || prepare(10000, Some(0)),
                |(mut world, mut dispatcher)| {
                    dispatcher.run(&mut world);
                },
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(
    benches,
    bench_create_transforms,
    bench_add_remove_components,
    par_bench_add_remove_components
);
criterion_main!(benches);
