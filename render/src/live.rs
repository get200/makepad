#![allow(unused_variables)]
use crate::cx::*;
use makepad_live_parser::LiveValue;
use makepad_live_parser::LiveDocument;
use makepad_live_parser::MultiPack;

#[derive(Clone, Debug)]
pub struct LiveBody {
    pub file: String,
    pub module_path: String,
    pub line: usize,
    pub column: usize,
    pub code: String,
    pub live_types: Vec<LiveType>
}

pub trait LiveFactory {
    fn live_new(&self, cx: &mut Cx) -> Box<dyn LiveComponent>;
    fn live_fields(&self, fields: &mut Vec<LiveField>);
    //    fn live_type(&self) -> LiveType;
}

pub trait LiveNew {
    fn live_new(cx: &mut Cx) -> Self;
    fn live_type() -> LiveType;
    fn live_register(cx: &mut Cx);
}

pub trait ToGenValue {
    fn to_gen_value(&self) -> GenValue;
}

pub trait LiveComponentValue {
    fn live_update_value(&mut self, cx: &mut Cx, id: Id, ptr: LivePtr);
    fn apply_value(&mut self, cx: &mut Cx, ndex: &mut usize, nodes: &[GenNode]);
}

pub trait LiveComponent {
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr);
    fn apply_index(&mut self, cx: &mut Cx, index: &mut usize, nodes: &[GenNode]);
    fn apply(&mut self, cx: &mut Cx, nodes: &[GenNode]) {
        if nodes.len()>2 {
            self.apply_index(cx, &mut 0, nodes);
        }
    }
}

pub trait CanvasComponent: LiveComponent {
    fn handle(&mut self, cx: &mut Cx, event: &mut Event);
    fn draw(&mut self, cx: &mut Cx);
    fn apply_draw(&mut self, cx: &mut Cx, nodes: &[GenNode]) {
        self.apply(cx, nodes);
        self.draw(cx);
    }
}

pub trait LiveComponentHooks {
    fn live_update_value_unknown(&mut self, _cx: &mut Cx, _id: Id, _ptr: LivePtr) {}
    fn apply_value_unknown(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        nodes.skip_value(index);
    }
    fn before_live_update(&mut self, _cx: &mut Cx, _live_ptr: LivePtr) {}
    fn after_live_update(&mut self, _cx: &mut Cx, _live_ptr: LivePtr) {}
    fn before_apply_index(&mut self, _cx: &mut Cx, _index: usize, _nodes: &[GenNode]) {}
    fn after_apply_index(&mut self, _cx: &mut Cx, _index: usize, _nodes: &[GenNode]) {}
}

pub enum LiveFieldKind {
    Local,
    Live,
}

pub struct LiveField {
    pub id: Id,
    pub live_type: Option<LiveType>,
    pub kind: LiveFieldKind
}

#[derive(Default)]
pub struct LiveBinding {
    pub live_ptr: Option<LivePtr>
}


impl Cx {
    pub fn live_register(&mut self) {
        crate::drawquad::live_register(self);
        crate::drawcolor::live_register(self);
        crate::drawtext::live_register(self);
        crate::geometrygen::live_register(self);
        crate::shader_std::live_register(self);
        crate::font::live_register(self);
        crate::turtle::live_register(self);
        crate::animation::live_register(self);
    }
    
    pub fn find_enum_origin(&self, start: MultiPack, lhs: Id) -> Option<(Id, Id)> {
        match start.unpack() {
            MultiUnpack::LivePtr(live_ptr) => {
                // ok so. the final live_ptr on an enum points to the rust_type
                let doc = &self.shader_registry.live_registry.expanded[live_ptr.file_id.to_index()];
                let node = &doc.nodes[live_ptr.local_ptr.level][live_ptr.local_ptr.index];
                if node.id == id!(rust_type) { // its our final pointer
                    if let LiveValue::LiveType(enum_type) = node.value {
                        // ok now we can look up the enum
                        if let Some(enum_info) = self.live_enums.get(&enum_type) {
                            return Some((enum_info.base_name, lhs));
                        }
                    }
                    else {
                        panic!()
                    }
                }
                match node.value {
                    LiveValue::MultiPack(id) => {
                        return self.find_enum_origin(id, node.id)
                    }
                    LiveValue::Class {class, ..} => {
                        return self.find_enum_origin(class, node.id)
                    },
                    LiveValue::Call {target, ..} => {
                        return self.find_enum_origin(target, node.id)
                    },
                    _ => ()
                }
            }
            _ => ()
        }
        // ok so we finally found our endpoint.
        None
    }
    
    pub fn live_ptr_from_id(&self, path: &str, id: Id) -> LivePtr {
        self.shader_registry.live_registry.live_ptr_from_path(
            ModulePath::from_str(path).unwrap(),
            &[id]
        ).unwrap()
    }
    
    pub fn resolve_ptr(&self, live_ptr: LivePtr) -> &LiveNode {
        self.shader_registry.live_registry.resolve_ptr(live_ptr)
    }
    
    pub fn resolve_doc_ptr(&self, live_ptr: LivePtr) -> (&LiveDocument, &LiveNode) {
        self.shader_registry.live_registry.resolve_doc_ptr(live_ptr)
    }
    
    
    pub fn find_class_prop_ptr(&self, class_ptr: LivePtr, seek_id: Id) -> Option<LivePtr> {
        if let Some(mut iter) = self.shader_registry.live_registry.live_class_iterator(class_ptr) {
            while let Some((id, live_ptr)) = iter.next_id(&self.shader_registry.live_registry) {
                if id == seek_id {
                    return Some(live_ptr)
                }
            }
        }
        None
    }
    
    // ok so now what. now we should run the expansion
    pub fn live_expand(&mut self) {
        // lets expand the f'er
        let mut errs = Vec::new();
        self.shader_registry.live_registry.expand_all_documents(&mut errs);
        for err in errs {
            println!("Error expanding live file {}", err);
        }
    }
    
    pub fn verify_type_signature(&self, live_ptr: LivePtr, live_type: LiveType) -> bool {
        let node = self.shader_registry.live_registry.resolve_ptr(live_ptr);
        if let LiveValue::LiveType(ty) = node.value {
            if ty == live_type {
                return true
            }
        }
        println!("TYPE SIGNATURE VERIFY FAILED");
        false
    }
    
    pub fn register_live_body(&mut self, live_body: LiveBody) {
        // ok so now what.
        //println!("{}", live_body.code);
        //let cm = CrateModule::from_module_path_check(&live_body.module_path).unwrap();
        //println!("register_live_body: {}", ModulePath::from_str(&live_body.module_path).unwrap());
        // ok so here we parse the live file
        
        let result = self.shader_registry.live_registry.parse_live_file(
            &live_body.file,
            ModulePath::from_str(&live_body.module_path).unwrap(),
            live_body.code,
            live_body.live_types,
            &self.live_enums,
            live_body.line
        );
        if let Err(msg) = result {
            println!("Error parsing live file {}", msg);
        }
    }
    
    pub fn register_factory(&mut self, live_type: LiveType, factory: Box<dyn LiveFactory>) {
        self.live_factories.insert(live_type, factory);
    }
    
    pub fn register_enum(&mut self, live_type: LiveType, info: LiveEnumInfo) {
        self.live_enums.insert(live_type, info);
    }
    
    pub fn get_factory(&mut self, live_type: LiveType) -> &Box<dyn LiveFactory> {
        self.live_factories.get(&live_type).unwrap()
    }
}


#[macro_export]
macro_rules!live_primitive {
    ( $ ty: ident, $ default: expr, $ update: item, $ apply: item, $ to_gen_value: item) => {
        impl ToGenValue for $ ty {
            $ to_gen_value
        }
        impl LiveComponent for $ ty {
            $ update
                $ apply
        }
        impl LiveNew for $ ty {
            fn live_new(_cx: &mut Cx) -> Self {
                $ default
            }
            fn live_type() -> LiveType {
                LiveType(std::any::TypeId::of::< $ ty>())
            }
            fn live_register(cx: &mut Cx) {
                struct Factory();
                impl LiveFactory for Factory {
                    fn live_new(&self, cx: &mut Cx) -> Box<dyn LiveComponent> where Self: Sized {
                        Box::new( $ ty ::live_new(cx))
                    }
                    
                    fn live_fields(&self, _fields: &mut Vec<LiveField>) where Self: Sized {
                    }
                }
                cx.live_factories.insert( $ ty::live_type(), Box::new(Factory()));
            }
        }
    }
}

live_primitive!(
    KeyFrameValue,
    KeyFrameValue::None,
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::MultiPack(id) => {
                match id.unpack() {
                    MultiUnpack::SingleId(id) => {
                        *self = KeyFrameValue::Id(id)
                    },
                    MultiUnpack::LivePtr(ptr) => {
                        let other_node = cx.shader_registry.live_registry.resolve_ptr(ptr);
                        *self =  KeyFrameValue::Id(other_node.id);
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match nodes[*index].value {
            GenValue::Id(id) => {
                *self = KeyFrameValue::Id(id);
                *index += 1;
            }
            GenValue::Float(f)=>{
                *self = KeyFrameValue::Float(f);
                *index += 1;
            }
            GenValue::Vec2(f)=>{
                *self = KeyFrameValue::Vec2(f);
                *index += 1;
            }
            GenValue::Vec3(f)=>{
                *self = KeyFrameValue::Vec3(f);
                *index += 1;
            }
            GenValue::Color(f)=>{
                *self = KeyFrameValue::Vec4(Vec4::from_u32(f));
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        match self{
            Self::None => GenValue::None,
            Self::Float(v)=> GenValue::Float(*v),
            Self::Vec2(v)=> GenValue::Vec2(*v),
            Self::Vec3(v)=> GenValue::Vec3(*v),
            Self::Vec4(v)=> GenValue::Color(v.to_u32()),
            Self::Id(v)=> GenValue::Id(*v),
        }
    }
);

live_primitive!(
    Id,
    Id::empty(),
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::MultiPack(id) => {
                match id.unpack() {
                    MultiUnpack::SingleId(id) => {
                        *self = id
                    },
                    MultiUnpack::LivePtr(ptr) => {
                        let other_node = cx.shader_registry.live_registry.resolve_ptr(ptr);
                        *self = other_node.id;
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match nodes[*index].value {
            GenValue::Id(id) => {
                *self = id;
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        GenValue::Id(*self)
    }
);

live_primitive!(
    LivePtr,
    LivePtr {file_id: FileId(0), local_ptr: LocalPtr {level: 0, index: 0}},
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::MultiPack(id) => {
                match id.unpack() {
                    MultiUnpack::LivePtr(ptr) => {
                        *self = ptr;
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        nodes.skip_value(index)
    },
    fn to_gen_value(&self) -> GenValue {
        panic!()
    }
);

live_primitive!(
    f32,
    0.0f32,
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::Int(val) => *self = val as f32,
            LiveValue::Float(val) => *self = val as f32,
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match nodes[*index].value {
            GenValue::Float(val) => {
                *self = val as f32;
                *index += 1;
            }
            GenValue::Int(val) => {
                *self = val as f32;
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        GenValue::Float(*self as f64)
    }
);

live_primitive!(
    f64,
    0.0f64,
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::Int(val) => *self = val as f64,
            LiveValue::Float(val) => *self = val as f64,
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match nodes[*index].value {
            GenValue::Float(val) => {
                *self = val as f64;
                *index += 1;
            }
            GenValue::Int(val) => {
                *self = val as f64;
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        GenValue::Float(*self as f64)
    }
);


live_primitive!(
    Vec2,
    Vec2::default(),
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::Vec2(v) => *self = v,
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match nodes[*index].value {
            GenValue::Vec2(val) => {
                *self = val;
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        GenValue::Vec2(*self)
    }
);

live_primitive!(
    Vec3,
    Vec3::default(),
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::Vec3(v) => *self = v,
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match nodes[*index].value {
            GenValue::Vec3(val) => {
                *self = val;
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        GenValue::Vec3(*self)
    }
);


live_primitive!(
    Vec4,
    Vec4::default(),
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::Color(v) => *self = Vec4::from_u32(v),
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match nodes[*index].value {
            GenValue::Color(v) => {
                *self = Vec4::from_u32(v);
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        GenValue::Color(self.to_u32())
    }
);


live_primitive!(
    String,
    String::default(),
    fn live_update(&mut self, cx: &mut Cx, ptr: LivePtr) {
        let node = cx.shader_registry.live_registry.resolve_ptr(ptr);
        match node.value {
            LiveValue::String {string_start, string_count} => {
                let origin_doc = cx.shader_registry.live_registry.get_origin_doc_from_token_id(node.token_id);
                origin_doc.get_string(string_start, string_count, self);
            }
            _ => ()
        }
    },
    fn apply_index(&mut self, _cx: &mut Cx, index: &mut usize, nodes: &[GenNode]) {
        match &nodes[*index].value {
            GenValue::Str(v) => {
                *self = v.to_string();
                *index += 1;
            }
            GenValue::String(v) => {
                *self = v.clone();
                *index += 1;
            }
            _ => nodes.skip_value(index)
        }
    },
    fn to_gen_value(&self) -> GenValue {
        GenValue::String(self.clone())
    }
);
