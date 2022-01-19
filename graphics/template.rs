use crate::SpriteData;

#[derive(Clone, Copy, Debug)]
pub enum Sprite {
{{#sprites}}
    #[allow(non_camel_case_types)]
    {{name}},
{{/sprites}}
}

impl Sprite {
    pub fn get(self) -> SpriteData {
        use Sprite::*;
        match self {
{{#sprites}}
            {{name}} => SpriteData {
                width: {{width}},
                height: {{height}},
                flags: {{flags}},
                data: vec![ {{bytes}} ],
            },
{{/sprites}}
        }
    }
}

