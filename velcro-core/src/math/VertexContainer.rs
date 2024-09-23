#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

type IndexFunction  = fn(usize);
type VoidFunction = fn();
type BoolFunction = fn(bool);
#[derive(Debug, Copy, Clone)]
pub struct VertexContainer<Vertex>{
     _vertices:Vec<Vertex>,
    _add_callback:IndexFunction,
    _remove_callback:IndexFunction,
    _update_callback:IndexFunction,
    _set_callback:VoidFunction,
    _clear_callback:VoidFunction,
}

impl <Vertex> VertexContainer<Vertex> {

    pub fn new(add_callback:IndexFunction, remove_callback:IndexFunction,
               update_callback:IndexFunction, set_callback:VoidFunction,
               clear_callback:VoidFunction) ->VertexContainer<Vertex>{
        VertexContainer{
            _vertices: vec![],
            _add_callback: add_callback.to_owned(),
            _remove_callback: remove_callback.to_owned(),
            _update_callback: update_callback.to_owned(),
            _set_callback: set_callback.to_owned(),
            _clear_callback: clear_callback.to_owned(),
        }
    }
    pub fn add_vertex(&mut self, vertex:&Vertex){
        self._vertices.push_back(vertex);

        if self._add_callback
        {
            self._add_callback(self._vertices.len() - 1);
        }
    }

    pub fn update_vertex(&mut self,index:usize,vertex:&Vertex)->bool{
        if index < self._vertices.len()
        {
            self._vertices[index] = vertex;
            if self._update_callback
            {
                self._update_callback(index);
            }
            return true;
        }
        return false;
    }

    pub fn insert_vertex(&mut self,index:usize,vertex:&Vertex)->bool{
        if index < self._vertices.len()
        {
            self._vertices.insert(self._vertices.data() + index, vertex);

            if self._add_callback
            {
                self._add_callback(index);
            }

            return true;
        }

        return false;
    }

    pub fn remove_vertex(&mut self,index:usize)->bool{
        if index < self._vertices.len()
        {
            self._vertices.remove(index);

            if self._remove_callback
            {
                self._remove_callback(index);
            }

            return true;
        }

        return false;
    }

    pub fn set_vertices(&mut self,vertices:&Vec<Vertex>){
        self._vertices.clear();
        for val in vertices{
            self._vertices.push(val)
        }
        if self._set_callback
        {
            self._set_callback();
        }
    }

    pub fn clear(&mut self){
        self._vertices.clear();

        if self._clear_callback
        {
            self._clear_callback();
        }
    }

    pub fn get_vertex(self,index:usize,mut vertex:&Vertex) ->bool{
        if index < self._vertices.len()
        {
            vertex = self._vertices[index].borrow_mut();
            return true;
        }
        return false;
    }

    pub fn get_last_vertex(self,mut vertex:&Vertex)->bool{
        if self._vertices.len() > 0
        {
            vertex = self._vertices[self._vertices.len()-1].borrow_mut();
            return true;
        }

        return false;
    }

    pub fn size(self)->usize{
        self._vertices.len()
    }

    pub fn empty(self)->bool{
        self._vertices.is_empty()
    }

    pub fn get_vertices(self)->&'static Vec<Vertex>{
        &self._vertices
    }

    pub fn set_callbacks(&mut self,add_callback:IndexFunction, remove_callback:IndexFunction,
                         update_callback:IndexFunction, set_callback:VoidFunction,
                         clear_callback:VoidFunction){
        self._add_callback =add_callback.to_owned();
        self._remove_callback = remove_callback.to_owned();
        self._update_callback = update_callback.to_owned();
        self._set_callback = set_callback.to_owned();
        self._clear_callback = clear_callback.to_owned();
    }

    fn add_notify(&mut self){
        let vertex_count = self._vertices.len();
        if vertex_count > 0
        {
            let last_vertex = vertex_count - 1;
            if vertex_count > 1
            {
                self._vertices[last_vertex] = self._vertices[vertex_count - 2].to_owned();
            }
            else
            {
                self._vertices[last_vertex] = Vertex::create_zero();
            }

            if self._add_callback
            {
                self._addCallback(last_vertex);
            }
        }
    }

    fn remove_notify(self, index:i32){
        if self._remove_callback
        {
            self._removeCallback(index);
        }
    }
    fn update_notify(self, index:i32){
        if self._update_callback
        {
            self._update_callback(index);
        }
    }
}