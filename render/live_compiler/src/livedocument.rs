#![allow(unused_variables)]
//use makepad_id_macros2::*;
use crate::id::{Id};
use std::fmt;
use crate::span::Span;
//use crate::util::PrettyPrintedF64;
use crate::token::{TokenWithSpan, TokenId};
use crate::livenode::{LiveNode, LiveValue};//, LiveValue};
//use crate::id::ModulePath;
use crate::id::LocalPtr;
use crate::id::LivePtr;
use crate::id::FileId;

pub struct LiveDocument {
    pub recompile: bool,
    pub nodes: Vec<LiveNode >,
    pub strings: Vec<char>,
    pub tokens: Vec<TokenWithSpan>,
    pub scopes: Vec<LiveScopeItem>,
}

impl fmt::Display for LiveScopeTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiveScopeTarget::LocalPtr {..} => {
                write!(f, "[local]")
            },
            LiveScopeTarget::LivePtr (ptr) => {
                write!(f, "[F:{} I:{}]", ptr.file_id.to_index(), ptr.local_ptr.0)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum LiveScopeTarget {
    LocalPtr(LocalPtr),
    LivePtr(LivePtr)
}

impl LiveScopeTarget {
    pub fn to_full_node_ptr(&self, file_id: FileId) -> LivePtr {
        match self {
            LiveScopeTarget::LocalPtr(local_ptr) => {
                LivePtr {file_id: file_id, local_ptr: *local_ptr}
            }
            LiveScopeTarget::LivePtr(live_ptr) => {
                *live_ptr
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct LiveScopeItem {
    pub id: Id,
    pub target: LiveScopeTarget
}


impl LiveDocument {
    pub fn new() -> Self {
        Self {
            recompile: true,
            nodes: Vec::new(),
            strings: Vec::new(),
            tokens: Vec::new(),
            scopes: Vec::new(),
        }
    }
    
    pub fn resolve_ptr(&self, local_ptr: LocalPtr) -> &LiveNode {
        &self.nodes[local_ptr.0]
    }
    
    pub fn get_tokens(&self, token_start: usize, token_count: usize) -> &[TokenWithSpan] {
        &self.tokens[token_start..(token_start + token_count)]
    }
    
    pub fn get_scopes(&self, scope_start: usize, scope_count: u32) -> &[LiveScopeItem] {
        &self.scopes[scope_start..(scope_start + scope_count as usize)]
    }
    
    pub fn get_string(&self, string_start: usize, string_count: usize, out:&mut String) {
        let chunk = &self.strings[string_start..(string_start + string_count)];
        out.truncate(0);
        for chr in chunk {
            out.push(*chr);
        }
    }
    
    pub fn restart_from(&mut self, other: &LiveDocument) {
        self.nodes.truncate(0);
        //self.multi_ids.clone_from(&other.multi_ids.clone());
        self.strings.clone_from(&other.strings);
        self.tokens.clone_from(&other.tokens.clone());
        self.scopes.truncate(0);
    }
    
    pub fn token_id_to_span(&self, token_id: TokenId) -> Span {
        self.tokens[token_id.token_index() as usize].span
    }
    
    pub fn scan_for_object_path_from(&self, object_path: &[Id], start:LocalPtr) -> Option<LocalPtr> {
        /*
        for i in 0..object_path.len() {
            let id = object_path[i];
            let mut found = false;
            for j in 0..node_count {
                let node = &self.nodes[level][j + node_start];
                if node.id == id {
                    // we found the node.
                    if i == object_path.len() - 1 { // last item
                        return Some(LocalPtr {
                            level: level,
                            index: j + node_start
                        });
                    }
                    else { // we need to be either an object or a class
                        level += 1;
                        match node.value {
                            LiveValue::Class {node_start: ns, node_count: nc, ..} => {
                                node_start = ns as usize;
                                node_count = nc as usize;
                            },
                            _ => return None
                            //LiveError {
                            //   span:self.token_id_to_span(token_id),
                            //   message: format!("Cannont find property {} is not an object path", IdFmt::dot(&multi_ids, Id::multi(id_start, id_count)))
                            // })
                        }
                        found = true;
                        break
                    }
                }
            }
            if !found {
                return None
            }
        }*/
        None
    }
    
    pub fn scan_for_object_path(&self, object_path: &[Id]) -> Option<LocalPtr> {
        self.scan_for_object_path_from(object_path, LocalPtr(0))
    }
    
    /*
    pub fn scan_for_multi_for_expand(&self, level: usize, node_start: usize, node_count: usize, id_start: usize, id_count: usize, multi_ids: &Vec<Id>) -> Result<LocalPtr, String> {
        let mut node_start = node_start as usize;
        let mut node_count = node_count as usize;
        let mut level = level;
        for i in 1..id_count {
            let id = multi_ids[i + id_start];
            let mut found = false;
            for j in 0..node_count {
                let node = &self.nodes[level][j + node_start];
                if node.id == id {
                    // we found the node.
                    if i == id_count - 1 { // last item
                        return Ok(LocalPtr {
                            level: level,
                            index: j + node_start
                        });
                    }
                    else { // we need to be either an object or a class
                        level += 1;
                        match node.value {
                            LiveValue::Class {node_start: ns, node_count: nc, ..} => {
                                node_start = ns as usize;
                                node_count = nc as usize;
                            },
                            _ => return Err(format!("Cannont find property {} is not an object path", MultiFmt::new(&multi_ids, MultiPack::multi_id(id_start, id_count))))
                            //LiveError {
                            //   span:self.token_id_to_span(token_id),
                            //   message: format!("Cannont find property {} is not an object path", IdFmt::dot(&multi_ids, Id::multi(id_start, id_count)))
                            // })
                        }
                        found = true;
                        break
                    }
                }
            }
            if !found {
                return Err(format!("Cannot find class {}", MultiFmt::new(&multi_ids, MultiPack::multi_id(id_start, id_count))))
            }
        }
        return Err(format!("Cannot find class {}", MultiFmt::new(&multi_ids, MultiPack::multi_id(id_start, id_count))))
    }*/
    /*
   pub fn write_or_add_node(
        &mut self,
        level: usize,
        node_start: usize,
        node_count: usize,
        in_doc: &LiveDocument,
        in_node: &LiveNode
    ) -> Result<Option<usize>, LiveError> {
        // I really need to learn to learn functional programming. This is absurd
        match in_node.id_pack.unpack() {
            IdUnpack::Multi {index: id_start, count: id_count} => {
                let mut node_start = node_start;
                let mut node_count = node_count;
                let mut level = level;
                let mut last_class = None;
                for i in 0..id_count {
                    let id = in_doc.multi_ids[i + id_start];
                    let mut found = false;
                    for j in 0..node_count {
                        let node = &mut self.nodes[level][j + node_start];
                        if node.id_pack == IdPack::single(id) {
                            // we found the node.
                            if i == id_count - 1 { // last item
                                // ok now we need to replace this node
                                
                                if node.value.get_type_nr() != in_node.value.get_type_nr() {
                                    if node.value.is_var_def() { // we can replace a vardef with something else
                                        continue;
                                    }
                                    // we cant replace a VarDef with something else
                                    return Err(LiveError {
                                        origin: live_error_origin!(),
                                        span: in_doc.token_id_to_span(in_node.token_id),
                                        message: format!("Cannot inherit with different node type {}", IdFmt::dot(&in_doc.multi_ids, in_node.id_pack))
                                    })
                                }
                                
                                node.token_id = in_node.token_id;
                                node.value = in_node.value;
                                return Ok(None)
                            }
                            else { // we need to be either an object or a class
                                level += 1;
                                match node.value {
                                    LiveValue::Class {node_start: ns, node_count: nc, ..} => {
                                        last_class = Some(j + node_start);
                                        node_start = ns as usize;
                                        node_count = nc as usize;
                                    },
                                    _ => return Err(LiveError {
                                        origin: live_error_origin!(),
                                        span: in_doc.token_id_to_span(in_node.token_id),
                                        message: format!("Setting property {} is not an object path", IdFmt::dot(&in_doc.multi_ids, in_node.id_pack))
                                    })
                                }
                                found = true;
                                break
                            }
                        }
                    }
                    if !found { //
                        if i != id_count - 1 || last_class.is_none() { // not last item, so object doesnt exist
                            return Err(LiveError {
                                origin: live_error_origin!(),
                                span: in_doc.token_id_to_span(in_node.token_id),
                                message: format!("Setting property {} is not an object path", IdFmt::dot(&in_doc.multi_ids, in_node.id_pack))
                            })
                        }
                        let last_class = last_class.unwrap();
                        let nodes_len = self.nodes[level].len();
                        if nodes_len == node_start + node_count { // can append to level
                            if let LiveValue::Class {node_count, ..} = &mut self.nodes[level - 1][last_class].value {
                                *node_count += 1;
                            }
                        }
                        else { // have to move all levelnodes. Someday test this with real data and do it better (maybe shift the rest up)
                            let ns = if let LiveValue::Class {node_start, node_count, ..} = &mut self.nodes[level - 1][last_class].value {
                                let ret = *node_start;
                                *node_start = nodes_len as u32;
                                *node_count += 1;
                                ret
                            }
                            else {
                                return Err(LiveError {
                                    origin: live_error_origin!(),
                                    span: in_doc.token_id_to_span(in_node.token_id),
                                    message: format!("Unexpected problem 1 in overwrite_or_add_node with {}", IdFmt::dot(&in_doc.multi_ids, in_node.id_pack))
                                })
                            };
                            let nodes = &mut self.nodes[level];
                            for i in 0..node_count {
                                let node = nodes[i as usize + ns as usize];
                                nodes.push(node);
                            }
                        }
                        // for object, string and array make sure we copy the values
                        
                        // push the final node
                        self.nodes[level].push(LiveNode {
                            token_id: in_node.token_id,
                            id_pack: IdPack::single(in_doc.multi_ids[id_start + id_count - 1]),
                            value: in_node.value
                        });
                        return Ok(None)
                    }
                }
                return Err(LiveError {
                    origin: live_error_origin!(),
                    span: in_doc.token_id_to_span(in_node.token_id),
                    message: format!("Unexpected problem 2 in overwrite_or_add_node with {}", IdFmt::dot(&in_doc.multi_ids, in_node.id_pack))
                })
            }
            IdUnpack::Single(id) => {
                let nodes = &mut self.nodes[level];
                for i in node_start..nodes.len() {
                    if nodes[i].id_pack == in_node.id_pack { // overwrite and exit
                        // lets error if the overwrite value type changed.
                        if nodes[i].value.get_type_nr() != in_node.value.get_type_nr() {
                            if nodes[i].value.is_var_def() { // we can replace a vardef with something else
                                continue;
                            }
                            return Err(LiveError {
                                origin: live_error_origin!(),
                                span: in_doc.token_id_to_span(in_node.token_id),
                                message: format!("Cannot inherit with different node type {}", IdFmt::dot(&in_doc.multi_ids, in_node.id_pack))
                            })
                        }
                        nodes[i] = *in_node;
                        return Ok(None)
                    }
                }
                let index = nodes.len();
                nodes.push(*in_node);
                return Ok(Some(index))
            }
            IdUnpack::Empty => {
                let nodes = &mut self.nodes[level];
                let index = nodes.len();
                nodes.push(*in_node);
                return Ok(Some(index))
            },
            _ => {
                return Err(LiveError {
                    origin: live_error_origin!(),
                    span: in_doc.token_id_to_span(in_node.token_id),
                    message: format!("Unexpected id type {}", IdFmt::dot(&in_doc.multi_ids, in_node.id_pack))
                })
            }
        }
    }*/
    /*
    pub fn create_multi_id(&mut self, ids: &[Id]) -> MultiPack {
        let multi_index = self.multi_ids.len();
        for id in ids {
            self.multi_ids.push(*id);
        }
        MultiPack::multi_id(multi_index, ids.len())
    }
    
    pub fn clone_multi_id(&mut self, id: MultiPack, other_ids: &[Id]) -> MultiPack {
        match id.unpack() {
            MultiUnpack::MultiId {index, count} => {
                let multi_index = self.multi_ids.len();
                for i in 0..count {
                    self.multi_ids.push(other_ids[i + index]);
                }
                MultiPack::multi_id(multi_index, self.multi_ids.len() - multi_index)
            }
            _ => {
                id
            }
        }
    }
    
    pub fn use_ids_to_module_path(&self, use_ids: MultiPack, outer_crate_id: Id) -> ModulePath {
        match use_ids.unpack() {
            MultiUnpack::MultiId {index, count} if count >= 2 => {
                let crate_id = self.multi_ids[index];
                let crate_id = if crate_id == id!(crate) {
                    outer_crate_id
                }else {
                    crate_id
                };
                ModulePath(crate_id, self.multi_ids[index + 1])
            }
            _ => {
                panic!("Unexpected id type {:?}", use_ids.unpack())
            }
        }
    }*/
}


impl fmt::Debug for LiveDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stack_depth = 0;
        for node in &self.nodes{
            for _ in 0..stack_depth {
                write!(f, "= ").unwrap();
            }
            match &node.value{
               LiveValue::Str(s)=>{
                   writeln!(f, "{}: Str: {}", node.id, s).unwrap();
                },
                LiveValue::String(s)=>{
                    writeln!(f, "{}: <String> {}", node.id, s).unwrap();
                },
                LiveValue::StringRef {string_start, string_count}=>{
                    writeln!(f, "{}: <StringRef> string_start:{}, string_end:{}", node.id, string_start, string_count).unwrap();
                },
                LiveValue::Bool(v)=>{
                    writeln!(f, "{}: <Bool> {}", node.id, v).unwrap();
                }
                LiveValue::Int(v)=>{
                    writeln!(f, "{}: <Int> {}", node.id, v).unwrap();
                }
                LiveValue::Float(v)=>{
                    writeln!(f, "{}: <Float> {}", node.id, v).unwrap();
                },
                LiveValue::Color(v)=>{
                    writeln!(f, "{}: Color:{}", node.id, v).unwrap();
                },
                LiveValue::Vec2(v)=>{
                    writeln!(f, "{}: Vec2:{:?}", node.id, v).unwrap();
                },
                LiveValue::Vec3(v)=>{
                    writeln!(f, "{}: Vec3:{:?}", node.id, v).unwrap();
                },
                LiveValue::LiveType(v)=>{
                    writeln!(f, "{}: <LiveType> {:?}", node.id, v).unwrap();
                },
                LiveValue::BareEnum {base, variant}=>{
                    writeln!(f, "{}: <BareEnum> {}::{}", node.id, base, variant).unwrap();
                },
                // stack items
                LiveValue::Array=>{
                    writeln!(f, "{}: <Array>", node.id).unwrap();
                    stack_depth += 1;
                },
                LiveValue::TupleEnum {base, variant}=>{
                    writeln!(f, "{}: <TupleEnum> {}::{}", node.id, base, variant).unwrap();
                    stack_depth += 1;
                },
                LiveValue::NamedEnum {base, variant}=>{
                    writeln!(f, "{}: <NamedEnum> {}::{}", node.id, base, variant).unwrap();
                    stack_depth += 1;
                },
                LiveValue::BareClass=>{
                    writeln!(f, "{}: <BareClass>", node.id).unwrap();
                    stack_depth += 1;
                }, // subnodes including this one
                LiveValue::NamedClass {class}=>{
                    writeln!(f, "{}: <NamedClass> {}", node.id, class).unwrap();
                    stack_depth += 1;
                }, // subnodes including this one
                LiveValue::Close=>{
                    writeln!(f, "<Close> {}", node.id).unwrap();
                    stack_depth -= 1;
                },
                // the shader code types
                LiveValue::Fn {
                    token_start,
                    token_count,
                    scope_start,
                    scope_count
                }=>{
                    writeln!(f, "fn {} :token_start:{}, token_count:{}, scope_start:{}, scope_end:{}", node.id, token_start, token_count, scope_start,scope_count).unwrap();
                },
                LiveValue::Const {
                    token_start,
                    token_count,
                    scope_start,
                    scope_count
                }=>{
                    writeln!(f, "const {} :token_start:{}, token_count:{}, scope_start:{}, scope_end:{}", node.id, token_start, token_count, scope_start,scope_count).unwrap();
                },
                LiveValue::VarDef { //instance/uniform def
                    token_start,
                    token_count,
                    scope_start,
                    scope_count
                }=>{
                    writeln!(f, "VarDef {} : token_start:{}, token_count:{}, scope_start:{}, scope_end:{}", node.id, token_start, token_count, scope_start,scope_count).unwrap();
                },
                LiveValue::Use{
                    crate_id,
                    module_id,
                    object_id
                }=>{
                    writeln!(f, "use {}::{}::{}", crate_id, module_id, object_id).unwrap();
                }
                
            }
        }
        fmt::Result::Ok(())
    }
}