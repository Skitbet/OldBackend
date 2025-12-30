use std::collections::HashSet;

pub fn toggle_reaction(
    likes: &mut HashSet<String>,
    dislikes: &mut HashSet<String>,
    user_id: &str,
    is_like: bool,
) -> bool {
    if is_like {
        let liked = likes.insert(user_id.to_string());
        if liked { dislikes.remove(user_id); }
        else { likes.remove(user_id); }
        liked
    } else {
        let disliked = dislikes.insert(user_id.to_string());
        if disliked { likes.remove(user_id); }
        else { dislikes.remove(user_id); }
        disliked
    }
}
