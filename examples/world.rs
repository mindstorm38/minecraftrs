use minecraftrs::version::Version;
use minecraftrs::world::World;
use minecraftrs::world::chunk::Chunk;

use std::fs::File;
use std::io::{Result as IoResult, prelude::*};
use std::time::Instant;


fn main() {

    // 3048926232851431861
    let mut world: World = World::new(3048926232851431861, Version::RELEASE_1_2_5);

    println!("World seed: {}", world.get_info().seed);

    //world.provide_chunk(9, -9).unwrap();

    let start = Instant::now();

    for x in 5..10 {
        for z in -7..-2 {
            world.provide_chunk(x, z).unwrap();
        }
    }

    println!("Generated {} chunks in {}s", world.get_chunks().len(), start.elapsed().as_secs_f32());

    let file = File::create("world.obj").unwrap();
    render_world_to_obj(file, &world).unwrap();

}


const MASK_NORTH: u8 = 0x1;
const MASK_SOUTH: u8 = 0x2;
const MASK_EAST: u8 = 0x4;
const MASK_WEST: u8 = 0x8;
const MASK_TOP: u8 = 0x10;
const MASK_BOTTOM: u8 = 0x20;


fn render_world_to_obj(mut file: File, world: &World) -> IoResult<()> {

    file.write_fmt(format_args!("# World export: {}\n\n", world.get_info().seed))?;
    file.write_fmt(format_args!("o world\n\n"))?;

    let mut index = 1;
    let mut objfaces: Vec<(usize, usize, usize)> = Vec::new();

    for chunk in world.get_chunks().values() {

        let (cx, cz) = chunk.get_position();

        println!("Rendering chunk at {}/{}", cx, cz);

        for y in 0..chunk.get_max_height() {
            for x in 0..16 {
                for z in 0..16 {

                    let (faces, bx, bz) = get_block_render(world, chunk, x, y, z);

                    if faces & MASK_NORTH == MASK_NORTH {
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 0, bz + 0))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 0, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 1, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 1, bz + 0))?;
                        objfaces.push((index + 0, index + 1, index + 2));
                        objfaces.push((index + 0, index + 2, index + 3));
                        index += 4;
                    }

                    if faces & MASK_SOUTH == MASK_SOUTH {
                        file.write_fmt(format_args!("v {} {} {}\n", bx, y + 0, bz + 0))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx, y + 0, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx, y + 1, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx, y + 1, bz + 0))?;
                        objfaces.push((index + 0, index + 1, index + 2));
                        objfaces.push((index + 0, index + 2, index + 3));
                        index += 4;
                    }

                    if faces & MASK_EAST == MASK_EAST {
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y + 0, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 0, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 1, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y + 1, bz + 1))?;
                        objfaces.push((index + 0, index + 1, index + 2));
                        objfaces.push((index + 0, index + 2, index + 3));
                        index += 4;
                    }

                    if faces & MASK_WEST == MASK_WEST {
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y + 0, bz))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 0, bz))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 1, bz))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y + 1, bz))?;
                        objfaces.push((index + 0, index + 1, index + 2));
                        objfaces.push((index + 0, index + 2, index + 3));
                        index += 4;
                    }

                    if faces & MASK_TOP == MASK_TOP {
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y + 1, bz + 0))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y + 1, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 1, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y + 1, bz + 0))?;
                        objfaces.push((index + 0, index + 1, index + 2));
                        objfaces.push((index + 0, index + 2, index + 3));
                        index += 4;
                    }

                    if faces & MASK_BOTTOM == MASK_BOTTOM {
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y, bz + 0))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 0, y, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y, bz + 1))?;
                        file.write_fmt(format_args!("v {} {} {}\n", bx + 1, y, bz + 0))?;
                        objfaces.push((index + 0, index + 1, index + 2));
                        objfaces.push((index + 0, index + 2, index + 3));
                        index += 4;
                    }

                }
            }
        }

    }

    file.write_all(b"\n")?;

    for face in objfaces {
        file.write_fmt(format_args!("f {} {} {}\n", face.0, face.1, face.2))?;
    }

    Ok(())

}


fn get_block_render(world: &World, chunk: &Chunk, x: usize, y: usize, z: usize) -> (u8, i32, i32) {

    let block = chunk.get_block_id(x, y, z);
    let (cx, cz) = chunk.get_position();
    let (cbx, cbz) = (cx << 4, cz << 4);

    let mut faces = 0;

    /*if block == 7 {
        println!("Bedrock at {}/{}/{}", x, y, z);
    }*/

    if block != 0 {

        let north_block = if x == 15 {
            world.get_block_id(cbx + 16, y as i32, cbz + z as i32).unwrap_or(u16::MAX)
        } else {
            chunk.get_block_id(x + 1, y, z)
        };

        let south_block = if x == 0 {
            world.get_block_id(cbx - 1, y as i32, cbz + z as i32).unwrap_or(u16::MAX)
        } else {
            chunk.get_block_id(x - 1, y, z)
        };

        let east_block = if z == 15 {
            world.get_block_id(cbx + x as i32, y as i32, cbz + 16).unwrap_or(u16::MAX)
        } else {
            chunk.get_block_id(x, y, z + 1)
        };

        let west_block = if z == 0 {
            world.get_block_id(cbx + x as i32, y as i32, cbz - 1).unwrap_or(u16::MAX)
        } else {
            chunk.get_block_id(x, y, z - 1)
        };

        let top_block = if y == chunk.get_max_height() - 1 {
            0
        } else {
            chunk.get_block_id(x, y + 1, z)
        };

        let bottom_block = if y == 0 {
            u16::MAX
        } else {
            chunk.get_block_id(x, y - 1, z)
        };

        if block != north_block && north_block != u16::MAX { faces |= MASK_NORTH; }
        if block != south_block && south_block != u16::MAX { faces |= MASK_SOUTH; }
        if block != east_block && east_block != u16::MAX { faces |= MASK_EAST; }
        if block != west_block && west_block != u16::MAX { faces |= MASK_WEST; }
        if block != top_block && top_block != u16::MAX { faces |= MASK_TOP; }
        if block != bottom_block && bottom_block != u16::MAX { faces |= MASK_BOTTOM; }

    }

    (faces, cbx + x as i32, cbz + z as i32)

}