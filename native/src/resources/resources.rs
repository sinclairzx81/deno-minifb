/*--------------------------------------------------------------------------

Deno-MiniFB

The MIT License (MIT)

Copyright (c) 2021 Haydn Paterson (sinclair) <haydn.developer@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

---------------------------------------------------------------------------*/

use std::any::Any;
use std::collections::HashMap;

pub trait Resource {
    fn any(&mut self) -> &mut dyn Any;
}

static mut RESOURCES: Option<HashMap<u32, Box<dyn Resource>>> = None;
static mut RESOURCE_ID: u32 = 0;

pub struct Resources; impl Resources {
    fn resolve<'a>() -> &'a mut HashMap<u32, Box<dyn Resource>> {
        unsafe {
            match RESOURCES {
                Some(ref mut store) => store,
                None => {
                    RESOURCES = Some(HashMap::new());
                    Resources::resolve()
                }
            }
        }
    }

    pub fn set<T: Resource + 'static>(resource: T) -> u32 {
        unsafe {
            let resource = Box::new(resource);
            let store = Resources::resolve();
            let handle = RESOURCE_ID;
            store.insert(handle, resource);
            RESOURCE_ID += 1;
            handle
        }
    }
    
    pub fn get<'a, T: Resource + 'static>(handle: u32) -> Option<&'a T> {
        let store = Resources::resolve();
        let reference = store.get_mut(&handle);
        match reference {
            None => None,
            Some(reference) => {
                let any = reference.any();
                any.downcast_ref::<T>()
            }
        }
    }

    pub fn get_mut<'a, T: Resource + 'static>(handle: u32) -> Option<&'a mut T> {
        let store = Resources::resolve();
        let reference = store.get_mut(&handle);
        match reference {
            None => None,
            Some(reference) => {
                let any = reference.any();
                any.downcast_mut::<T>()
            }
        }
    }
    
    pub fn delete<'a>(handle: u32) {
        let store = Resources::resolve();
        store.remove(&handle);
    }
}

