use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Context};
use itertools::Itertools;
use log::debug;

use crate::vertex::Vertex;

pub fn load_vertices_from_database(database_file: &str) -> Result<Vec<Vertex>, anyhow::Error> {
    let (db_entries, expected_entries) = load_data_from_file(database_file)?;
    let mut vertices = vec![Vertex {
        ..Default::default()
    }];
    let vertices_from_db: Vec<Vertex> =
        db_entries.map(convert_maybe_string_to_node).try_collect()?;
    if vertices_from_db.len() != expected_entries {
        bail!(
            "The number vertices ({}) isn't equal to the number declared: {expected_entries}",
            vertices_from_db.len()
        )
    }
    vertices.extend(vertices_from_db);

    Ok(vertices)
}

fn convert_maybe_string_to_node(
    (line_number, result): (usize, Result<String, std::io::Error>),
) -> Result<Vertex, anyhow::Error> {
    match result {
        Ok(string_line) => Vertex::from_str(string_line, line_number + 1),
        Err(err) => Err(anyhow::Error::from(err)),
    }
}

pub fn load_data_from_file(
    filename: impl AsRef<str>,
) -> Result<
    (
        impl Iterator<Item = (usize, Result<String, std::io::Error>)>,
        usize,
    ),
    anyhow::Error,
> {
    let file = File::open(filename.as_ref())?;
    let reader = BufReader::new(file);
    let mut lines_reader = reader.lines().enumerate();
    let number_of_entries = get_number_of_nodes(&mut lines_reader)?;

    Ok((lines_reader, number_of_entries))
}

fn get_number_of_nodes(
    buffer: &mut impl Iterator<Item = (usize, Result<String, std::io::Error>)>,
) -> Result<usize, anyhow::Error> {
    let first_line = buffer
        .next()
        .context("end of file")?
        .1
        .context("unable to get the first line")?;
    let number_of_nodes: usize = first_line.parse()?;

    debug!("Extracted number of nodes in graph: {number_of_nodes}");
    Ok(number_of_nodes)
}
