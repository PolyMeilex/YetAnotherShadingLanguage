use proc_macro::Span;

#[derive(Debug)]
pub enum Glsl {
    Fragment(GlslFragment),
    Expr(String),
    Line(GlslLine),
}
impl Glsl {
    pub fn to_string(&self) -> String {
        match self {
            Glsl::Fragment(frag) => frag.to_string(),
            Glsl::Expr(s) => s.to_owned(),
            Glsl::Line(l) => l.to_string(),
        }
    }
}
impl std::fmt::Display for Glsl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
#[derive(Debug)]
pub struct GlslLine {
    pub span: Option<Span>,
    pub ends_with_semi: bool,
    pub glsl_string: String,
}
impl GlslLine {
    pub fn to_string(&self) -> String {
        if self.ends_with_semi {
            self.glsl_string.clone() + ";\n"
        } else {
            self.glsl_string.clone() + "\n"
        }
    }
}

#[derive(Debug)]
pub struct GlslFragment {
    pub elements: Vec<Glsl>,
}
impl GlslFragment {
    pub fn to_string(&self) -> String {
        let mut out: String = String::new();

        for l in &self.elements {
            out += &l.to_string();
        }

        out
    }
    pub fn squash(self) -> Vec<GlslLine> {
        let mut lines = Vec::new();
        for e in self.elements.into_iter() {
            match e {
                Glsl::Line(l) => lines.push(l),
                Glsl::Fragment(f) => lines.append(&mut f.squash()),
                Glsl::Expr(_) => panic!("No Expr's allowe in toplevel"),
            }
        }
        lines
    }
}

pub trait AsGlsl {
    fn as_glsl(&self) -> Glsl;
}
