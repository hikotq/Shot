use glium;
use glium::Surface;
use object::Position;


pub struct Render<'a> {
    display: &'a glium::Display,
    target: Option<glium::Frame>,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

impl<'a> Render<'a> {
    pub fn new(display: &'a glium::Display) -> Self {
        let target = display.draw();
        Render {
            display: display,
            target: Some(target),
        }
    }
    pub fn clear_color(&mut self, r: f32, g: f32, b: f32, alpha: f32) {
        let target = self.target.as_mut().unwrap();
        target.clear_color(r, g, b, alpha);
    }

    pub fn finish(mut self) {
        if self.target.is_some() {
            let target = self.target.take();
            target.unwrap().finish().unwrap();
        } else {
            return;
        }
    }

    pub fn draw_rectangle(&mut self, pos: Position, radius: f32) {
        let (width, height) = self.display.get_framebuffer_dimensions();
        let pos = Position {
            x: pos.x / (width as f32 / 2.0) - 1.0,
            y: pos.y / (height as f32 / 2.0) - 1.0,
        };
        let vertex_buffer = glium::VertexBuffer::empty_dynamic(self.display, 4).unwrap();
        let indices = {
            let ib_data: Vec<u16> = vec![0, 1, 2, 1, 3, 2];
            let ib = glium::IndexBuffer::new(
                self.display,
                glium::index::PrimitiveType::TrianglesList,
                &ib_data,
            ).unwrap();
            ib
        };

        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;

        let left = pos.x - radius / half_width;
        let right = pos.x + radius / half_width;
        let top = pos.y + radius / half_height;
        let bottom = pos.y - radius / half_height;
        let vb_data = vec![
            Vertex { position: [left, top] },
            Vertex { position: [right, top] },
            Vertex { position: [left, bottom] },
            Vertex { position: [right, bottom] },
        ];
        vertex_buffer.write(&vb_data);

        let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program =
            glium::Program::from_source(self.display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        self.target
            .as_mut()
            .unwrap()
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }

    pub fn draw_circle(&mut self, pos: Position, radius: f32, a: f32, b: f32) {
        let (width, height) = self.display.get_framebuffer_dimensions();
        let mut shape = Vec::new();
        let n: i32 = 200;
        for i in 0..n {
            let x = pos.x + a * radius * (2.0 * 3.1415 * (i as f32 / n as f32)).cos();
            let y = pos.y + b * radius * (2.0 * 3.1415 * (i as f32 / n as f32)).sin();
            let vertex = Vertex {
                position: [
                    x / (width as f32 / 2.0) - 1.0,
                    y / (height as f32 / 2.0) - 1.0,
                ],
            };
            shape.push(vertex);
        }

        let vertex_buffer = glium::VertexBuffer::new(self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program =
            glium::Program::from_source(self.display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        self.target
            .as_mut()
            .unwrap()
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
