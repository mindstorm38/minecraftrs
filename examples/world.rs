use minecraftrs::version::Version;
use minecraftrs::world::World;

use std::fs::File;
use std::io::{Result as IoResult, prelude::*};

fn main() {

    let mut world: World = World::new(3048926232851431861, Version::RELEASE_1_2_5);

    println!("World seed: {}", world.get_info().seed);

    world.provide_chunk(0, 10).unwrap();
    world.provide_chunk(1, 10).unwrap();
    world.provide_chunk(2, 10).unwrap();
    world.provide_chunk(3, 10).unwrap();

    //let file = File::create("world.obj").unwrap();
    //render_world_to_obj(file, &world).unwrap();

}

/*fn render_world_to_obj(mut file: File, world: &World) -> IoResult<()> {

    file.write_fmt(format_args!("# World export: {}\n\n", world.get_info().seed))?;
    file.write_fmt(format_args!("o world\n\n"))?;

    let mut index = 1;
    let mut faces: Vec<(usize, usize, usize)> = Vec::new();

    for chunk in world.get_chunks().values() {

        let (cx, cz) = chunk.get_position();
        let sub_chunks = chunk.get_sub_chunks();

        println!("Rendering chunk at {}/{}", cx, cz);

        for (cy, sub_chunk) in sub_chunks.iter().enumerate() {

            for x in 0..16 {
                for z in 0..16 {
                    for y in 0..16 {

                        let block = sub_chunk.get_block_raw(x, y, z);

                        if block != 0 {

                            let bx = cx * 16 + x as i32;
                            let by = (cy * 16 + y) as i32;
                            let bz = cz * 16 + z as i32;

                            if x == 0 || sub_chunk.get_block_raw(x - 1, y, z) != block {
                                file.write_fmt(format_args!("v {} {} {}\n", bx, by + 0, bz + 0))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx, by + 0, bz + 1))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx, by + 1, bz + 1))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx, by + 1, bz + 0))?;
                                faces.push((index + 0, index + 1, index + 2));
                                faces.push((index + 0, index + 2, index + 3));
                                index += 4;
                            }

                            if y + 1 == 16 || sub_chunk.get_block_raw(x, y + 1, z) != block {
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 0, by + 1, bz + 0))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 0, by + 1, bz + 1))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 1, by + 1, bz + 1))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 1, by + 1, bz + 0))?;
                                faces.push((index + 0, index + 1, index + 2));
                                faces.push((index + 0, index + 2, index + 3));
                                index += 4;
                            }

                            /*if y + 1 == 16 || sub_chunk.get_block_raw(x, y - 1, z) != block {
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 0, by, bz + 0))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 0, by, bz + 1))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 1, by, bz + 1))?;
                                file.write_fmt(format_args!("v {} {} {}\n", bx + 1, by, bz + 0))?;
                                faces.push((index + 0, index + 1, index + 2));
                                faces.push((index + 0, index + 2, index + 3));
                                index += 4;
                            }*/

                        }

                    }
                }
            }

        }
    }

    file.write_all(b"\n")?;

    for face in faces {
        file.write_fmt(format_args!("f {} {} {}\n", face.0, face.1, face.2))?;
    }

    Ok(())

}*/