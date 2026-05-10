pub use studycycle_db_schema::{
  newtypes::CommentId,
  source::comment::{Comment, CommentActions, CommentInsertForm},
};
pub use studycycle_db_views_comment::{
  CommentSlimView,
  CommentView,
  api::{CommentResponse, GetComment, GetComments},
};

pub mod actions {
  pub use studycycle_db_views_comment::api::{
    CreateComment,
    CreateCommentLike,
    DeleteComment,
    EditComment,
    SaveComment,
  };

  pub mod moderation {
    pub use studycycle_db_views_comment::api::{
      DistinguishComment,
      ListCommentLikes,
      PurgeComment,
      RemoveComment,
    };
  }
}
