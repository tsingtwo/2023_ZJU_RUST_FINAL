#![feature(impl_trait_in_assoc_type)]
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};
use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};
use anyhow::{anyhow, Error};
use pilota::FastStr;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use volo_gen::volo::example::ItemServiceClient;
use core::result::Result;
pub struct S{
	pub mst: Arc<Mutex<Vec<ItemServiceClient>>>,
	pub mas: Arc<Mutex<Vec<Vec<ItemServiceClient>>>>,
	pub mst_and_key: Arc<Mutex<HashMap<String,ItemServiceClient>>>
}
impl S {
	pub fn new()->S{
		S { mst: Arc::new(Mutex::new(Vec::new())), mas:Arc::new(Mutex::new(Vec::new())), mst_and_key: Arc::new(Mutex::new(HashMap::new()))}
		// 这里尝试过mas用hashmap，但是在下面两个client需要比较的时候发现没办法比较，所以只能换回vec统一索引来保证一致
	}
}
#[allow(unused)]
#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: volo_gen::volo::example::GetItemRequest,
    ) -> core::result::Result<volo_gen::volo::example::GetItemResponse, volo_thrift::AnyhowError>
    {
		
		let mut resp = volo_gen::volo::example::GetItemResponse{op: " ".into(),key: _req.key.clone(), val: _req.val.clone(), status: false};
        let k = _req.key.to_string();
		let mut hash = DefaultHasher::new();
		let hash_code = {_req.key.clone().as_str().hash(&mut hash); hash.finish()};
		let mut a:ItemServiceClient;
		if self.mst_and_key.lock().unwrap().contains_key(&k) {
			a = self.mst_and_key.lock().unwrap().get(&k).unwrap().clone();
		}else{
			let mn = self.mst.lock().unwrap().len();
			println!("{}", mn);
			let num_c = (hash_code as usize) % mn;


			let m_chosen = self.mst.lock().unwrap().get(num_c).unwrap().clone();
			if _req.op == "get" {
				let _mn = self.mas.lock().unwrap().clone()[num_c].len();
				let _num_c = (hash_code as usize) % _mn;
				a = self.mas.lock().unwrap().clone()[num_c][_num_c].clone();
				self.mst_and_key.lock().unwrap().insert(k, a.clone());
			}else{
				a = m_chosen;
				self.mst_and_key.lock().unwrap().insert(k, a.clone());
			}
		}
		// println!("{}, {}, {}", _req.op.clone(), _req.key.clone(), _req.val.clone());
		
		match a.get_item(_req).await {
			Ok(resp) =>Ok(resp),
			Err(e)=>Err(Error::from(anyhow::Error::msg(e)))
		}
		
	}
}

pub struct FilterLayer;
impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}
#[derive(Clone)]
pub struct FilterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    Cx: Send + 'static,
	anyhow::Error: Into<S::Error>,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let info = format!("{req:?}");
		let mut ill = true;
		if info.contains("尊尼获嘉") {
			ill = false;
		} 
		if ill {
			let resp =self.0.call(cx, req).await;
			resp
		} else {
			Err(anyhow!("给你房管你给我说话").into())
		}
    }
}

