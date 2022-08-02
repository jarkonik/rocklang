use crate::{
    llvm::{self, Context},
    parser,
};

pub fn get_llvm_type(context: &Context, typ: &parser::Type) -> llvm::Type {
    match typ {
        parser::Type::Vector => context.void_type().pointer_type(0),
        parser::Type::Numeric => context.double_type(),
        parser::Type::Function => todo!(),
        parser::Type::Void => context.void_type(),
        parser::Type::Ptr => context.void_type().pointer_type(0),
        parser::Type::String => context.i8_type().pointer_type(0),
        parser::Type::Bool => context.i1_type(),
    }
}
