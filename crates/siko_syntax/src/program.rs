use crate::actor::Actor;
use crate::actor::ActorId;
use crate::actor::Protocol;
use crate::actor::ProtocolId;
use crate::class::Class;
use crate::class::ClassId;
use crate::class::Instance;
use crate::class::InstanceId;
use crate::data::Adt;
use crate::data::AdtId;
use crate::data::Record;
use crate::data::RecordField;
use crate::data::RecordFieldId;
use crate::data::RecordId;
use crate::data::Variant;
use crate::data::VariantId;
use crate::expr::Expr;
use crate::expr::ExprId;
use crate::function::Function;
use crate::function::FunctionId;
use crate::function::FunctionType;
use crate::function::FunctionTypeId;
use crate::import::Import;
use crate::import::ImportId;
use crate::module::Module;
use crate::module::ModuleId;
use crate::pattern::Pattern;
use crate::pattern::PatternId;
use crate::types::TypeSignature;
use crate::types::TypeSignatureId;
use siko_location_info::item::ItemInfo;
use siko_util::ItemContainer;

#[derive(Debug)]
pub struct Program {
    pub modules: ItemContainer<ModuleId, Module>,
    pub functions: ItemContainer<FunctionId, Function>,
    pub function_types: ItemContainer<FunctionTypeId, FunctionType>,
    pub records: ItemContainer<RecordId, Record>,
    pub adts: ItemContainer<AdtId, Adt>,
    pub variants: ItemContainer<VariantId, Variant>,
    pub classes: ItemContainer<ClassId, Class>,
    pub instances: ItemContainer<InstanceId, Instance>,
    pub exprs: ItemContainer<ExprId, ItemInfo<Expr>>,
    pub type_signatures: ItemContainer<TypeSignatureId, ItemInfo<TypeSignature>>,
    pub patterns: ItemContainer<PatternId, ItemInfo<Pattern>>,
    pub imports: ItemContainer<ImportId, Import>,
    pub record_fields: ItemContainer<RecordFieldId, RecordField>,
    pub protocols: ItemContainer<ProtocolId, Protocol>,
    pub actors: ItemContainer<ActorId, Actor>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            modules: ItemContainer::new(),
            functions: ItemContainer::new(),
            function_types: ItemContainer::new(),
            records: ItemContainer::new(),
            adts: ItemContainer::new(),
            variants: ItemContainer::new(),
            classes: ItemContainer::new(),
            instances: ItemContainer::new(),
            exprs: ItemContainer::new(),
            type_signatures: ItemContainer::new(),
            patterns: ItemContainer::new(),
            imports: ItemContainer::new(),
            record_fields: ItemContainer::new(),
            protocols: ItemContainer::new(),
            actors: ItemContainer::new(),
        }
    }
}
