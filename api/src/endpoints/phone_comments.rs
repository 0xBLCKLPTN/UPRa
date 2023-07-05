use salvo::prelude::*;
use crate::misc::models::*;
use crate::configurator::RB;

// Crud operattions.
impl_select!(Phones{select_by_name(phone_number: String) -> Option => "`where phone_number = #{phone_number} limit 1`"});
impl_select!(Comments{select_by_name(phone_id: i64) -> Vec => "`where phone_id = #{phone_id}`"});
crud!(Comments{});
crud!(Phones{});

#[handler]
pub async fn show_comments(req: &mut Request, res: &mut Response) {
    let phone_number = req.query::<String>("number").unwrap();
    let phone_data = Phones::select_by_name(&mut RB.clone(), phone_number.to_string()).await.unwrap();
    let phone_comments:Vec<Comments> = Comments::select_by_name(&mut RB.clone(), phone_data.unwrap().id.unwrap()).await.unwrap();
    
    let mut res_data = Vec::new();
    
    for comment in phone_comments.iter() {
        res_data.push(comment.comment.clone());
    }

    res.render(serde_json::to_string(&res_data).unwrap());
}


async fn add_number(pn: &str, commentary: String) {
    let _ = Phones::insert(&mut RB.clone(), &Phones{ id: None, phone_number: pn.to_string()} ).await;
    let pi = Phones::select_by_name(&mut RB.clone(), pn.to_string()).await.unwrap().unwrap().id.unwrap();

    add_commentary(pi, &commentary).await;
}

async fn add_commentary(pi: i64, comment: &str) {
    let _ = Comments::insert(&mut RB.clone(), &Comments {id: None, phone_id: pi, comment: comment.to_string()}).await;
}


#[handler]
pub async fn add_comment(req: &mut Request, res: &mut Response) {
    let phone_number = req.query::<String>("number").unwrap();
    let comment = req.query::<String>("comment").unwrap();

    let phone_id = Phones::select_by_name(&mut RB.clone(), phone_number.to_string()).await.unwrap();
    let _ = match phone_id {
        None => add_number(&phone_number, comment.to_string()).await,
        _ => add_commentary(phone_id.unwrap().id.unwrap(), &comment).await,
    };
    res.render(serde_json::to_string("{'status': 'done'}").unwrap());
}


#[handler]
pub async fn edit_comment(req: &mut Request) -> &'static str {
    "Hello World"
}
