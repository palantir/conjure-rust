use conjure_codegen::type_definition::TypeDefinition;

/// A wrapper type to help collect TypeDefinitions from recursive parsing calls.
#[derive(Debug)]
pub(crate) struct Hoist<T> {
    type_: T,
    hoist: Vec<TypeDefinition>,
}

impl<T> Hoist<T> {
    /// Create a new Hoist
    pub fn new(type_: T) -> Self {
        Self {
            type_,
            hoist: Vec::new(),
        }
    }

    /// Returns the wrapped type and an empty Hoist still containing the hoisted objects.
    pub fn explode(self) -> (T, Hoist<()>) {
        (
            self.type_,
            Hoist {
                type_: (),
                hoist: self.hoist,
            },
        )
    }

    /// Add a TypeDefinition to the Hoist.
    pub fn push(&mut self, hoist_: &TypeDefinition) {
        match hoist_ {
            TypeDefinition::Alias(_) => (),
            _ => self.hoist.push(hoist_.clone()),
        }
    }

    // Extends the hoisted stack. Returns the consumed type.
    pub fn extend<V>(&mut self, hoisted: Hoist<V>) -> V {
        self.hoist.extend(hoisted.hoist);
        hoisted.type_
    }
}

impl Hoist<()> {
    /// Wrap the given type creating a new typed Hoist with the exisiting hoisted objects.
    pub fn wrap<T>(self, type_: T) -> Hoist<T> {
        Hoist {
            type_,
            hoist: self.hoist,
        }
    }

    /// Returns the contained TypeDefinitions as an iterator.
    pub fn into_iter(self) -> impl Iterator<Item = TypeDefinition> {
        self.hoist.into_iter()
    }
}

impl Hoist<TypeDefinition> {
    /// Special case to turn Hoist<TypeDefinition> into Hoist<()> that contains itself.
    pub fn flatten(self) -> Hoist<()> {
        let mut hoist = self.hoist;
        hoist.push(self.type_);
        Hoist { type_: (), hoist }
    }
}
