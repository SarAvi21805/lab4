use raylib::prelude::*;
use std::fs;

mod line;

struct Obj {
    vertices: Vec<Vector3>,
    indices: Vec<usize>,
}

fn load_obj(path: &str) -> Result<Obj, String> {
    let contents =
        fs::read_to_string(path).map_err(|e| format!("No pude leer {path}: {e}"))?;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("v") => {
                let nums: Result<Vec<f32>, _> =
                    parts.take(3).map(str::parse::<f32>).collect();

                let nums = nums.map_err(|e| {
                    format!(
                        "Coordenada inválida en la línea {}: {e}",
                        line_index + 1
                    )
                })?;

                if nums.len() != 3 {
                    return Err(format!(
                        "Faltan coordenadas en la línea {}",
                        line_index + 1
                    ));
                }

                vertices.push(Vector3::new(nums[0], nums[1], nums[2]));
            }

            Some("f") => {
                let face: Result<Vec<usize>, String> = parts
                    .map(|token| {
                        // En tokens como 5/7/2, por ahora solo usamos el 5.
                        let first_number = token.split('/').next().unwrap_or("");

                        let obj_index = first_number.parse::<usize>().map_err(|e| {
                            format!(
                                "Índice inválido en la línea {}: {e}",
                                line_index + 1
                            )
                        })?;

                        // Manejo de índices porque OBJ empieza a contar desde 1 y Rust empieza desde 0
                        let index = obj_index.checked_sub(1).ok_or_else(|| {
                            format!(
                                "El índice 0 no es válido en OBJ (línea {})",
                                line_index + 1
                            )
                        })?;

                        if index >= vertices.len() {
                            return Err(format!(
                                "Índice fuera de rango en la línea {}",
                                line_index + 1
                            ));
                        }

                        Ok(index)
                    })
                    .collect();

                let face = face?;

                if face.len() < 3 {
                    return Err(format!(
                        "La cara necesita al menos 3 vértices (línea {})",
                        line_index + 1
                    ));
                }

                // Conversión de caras de 3 o más vértices en triángulos, para cada cara genera un triángulo
                for i in 1..face.len() - 1 {
                    indices.extend_from_slice(&[face[0], face[i], face[i + 1]]);
                }
            }

            // Ignora comentarios, materiales, normales y demás líneas
            _ => {}
        }
    }

    if vertices.is_empty() {
        return Err("El archivo no contiene vértices (líneas 'v').".to_string());
    }

    if indices.len() < 3 {
        return Err("El archivo no contiene caras válidas (líneas 'f').".to_string());
    }

    Ok(Obj { vertices, indices })
}

fn main() {
    const WIDTH: i32 = 1000;
    const HEIGHT: i32 = 800;

    let model = match load_obj("assets/nave.obj") {
        Ok(model) => model,
        Err(error) => {
            eprintln!("Error al cargar el OBJ: {error}");
            return;
        }
    };

    println!("Vértices cargados: {}", model.vertices.len());
    println!("Índices cargados: {}", model.indices.len());
    println!("Triángulos: {}", model.indices.len() / 3);

    // Encuentra los límites del modelo en los ejes X e Y
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for vertex in &model.vertices {
        min_x = min_x.min(vertex.x);
        max_x = max_x.max(vertex.x);
        min_y = min_y.min(vertex.y);
        max_y = max_y.max(vertex.y);
    }

    let mut span_x = max_x - min_x;
    let mut span_y = max_y - min_y;

    // Evita dividir por cero si el modelo no tiene extensión en un eje.
    if span_x.abs() < f32::EPSILON {
        span_x = 1.0;
    }
    if span_y.abs() < f32::EPSILON {
        span_y = 1.0;
    }

    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    // Uso 80 % del tamaño de la ventana
    let scale_x = (WIDTH as f32 * 0.8) / span_x;
    let scale_y = (HEIGHT as f32 * 0.8) / span_y;
    let scale = scale_x.min(scale_y);

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Modelo OBJ - dibujo de triángulos")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for tri in model.indices.chunks_exact(3) {
            let vertex_a = model.vertices[tri[0]];
            let vertex_b = model.vertices[tri[1]];
            let vertex_c = model.vertices[tri[2]];

            // Proyección X/Y: centra, escala e invierte Y porque la coordenada vertical de pantalla crece hacia abajo.
            let a = Vector2::new(
                (vertex_a.x - center_x) * scale + WIDTH as f32 / 2.0,
                HEIGHT as f32 / 2.0 - (vertex_a.y - center_y) * scale,
            );

            let b = Vector2::new(
                (vertex_b.x - center_x) * scale + WIDTH as f32 / 2.0,
                HEIGHT as f32 / 2.0 - (vertex_b.y - center_y) * scale,
            );

            let c = Vector2::new(
                (vertex_c.x - center_x) * scale + WIDTH as f32 / 2.0,
                HEIGHT as f32 / 2.0 - (vertex_c.y - center_y) * scale,
            );

            line::triangle(&mut d, a, b, c, Color::RAYWHITE);
        }

        d.draw_text(
            "Modelo OBJ: contorno de triangulos",
            20,
            20,
            20,
            Color::GREEN,
        );
    }
}