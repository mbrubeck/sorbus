use {
    crate::{green, ArcBorrow, TextSize},
    std::{
        fmt::{self, Debug},
        ops::Deref,
        sync::Arc,
    },
};

/// Raw kind tag for each element in the tree.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Kind(pub u16);

/// Skip multiline, just do it inline
impl Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kind({})", self.0)
    }
}

/// Enum wrapping either a node or a token.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NodeOrToken<Node, Token> {
    Node(Node),
    Token(Token),
}

#[allow(missing_docs)]
impl<Node, Token> NodeOrToken<Node, Token> {
    pub fn into_node(self) -> Option<Node> {
        self.map(Some, |_| None).flatten()
    }

    pub fn as_node(&self) -> Option<&Node> {
        self.as_ref().into_node()
    }

    pub fn is_node(&self) -> bool {
        self.as_node().is_some()
    }

    pub fn unwrap_node(self) -> Node {
        self.into_node().expect("called `unwrap_node` on token")
    }

    pub fn into_token(self) -> Option<Token> {
        self.map(|_| None, Some).flatten()
    }

    pub fn as_token(&self) -> Option<&Token> {
        self.as_ref().into_token()
    }

    pub fn is_token(&self) -> bool {
        self.as_token().is_some()
    }

    pub fn unwrap_token(self) -> Token {
        self.into_token().expect("called `unwrap_token` on node")
    }
}

#[allow(missing_docs)]
impl<Node, Token> NodeOrToken<Node, Token> {
    pub fn as_ref(&self) -> NodeOrToken<&Node, &Token> {
        match *self {
            NodeOrToken::Node(ref node) => NodeOrToken::Node(node),
            NodeOrToken::Token(ref token) => NodeOrToken::Token(token),
        }
    }

    pub(crate) fn map<N, T>(
        self,
        n: impl FnOnce(Node) -> N,
        t: impl FnOnce(Token) -> T,
    ) -> NodeOrToken<N, T> {
        match self {
            NodeOrToken::Node(node) => NodeOrToken::Node(n(node)),
            NodeOrToken::Token(token) => NodeOrToken::Token(t(token)),
        }
    }

    pub fn as_deref(&self) -> NodeOrToken<&Node::Target, &Token::Target>
    where
        Node: Deref,
        Token: Deref,
    {
        self.as_ref().map(Deref::deref, Deref::deref)
    }

    pub fn kind(&self) -> Kind
    where
        Node: Deref<Target = green::Node>,
        Token: Deref<Target = green::Token>,
    {
        self.as_deref().map(green::Node::kind, green::Token::kind).flatten()
    }
}

#[allow(missing_docs)]
#[allow(clippy::len_without_is_empty)]
impl<Node, Token> NodeOrToken<Node, Token>
where
    Node: Deref<Target = green::Node>,
    Token: Deref<Target = green::Token>,
{
    pub fn len(&self) -> TextSize {
        self.as_deref().map(green::Node::len, green::Token::len).flatten()
    }
}

impl<T> NodeOrToken<T, T> {
    pub(crate) fn flatten(self) -> T {
        match self {
            NodeOrToken::Node(node) => node,
            NodeOrToken::Token(token) => token,
        }
    }
}

impl From<Arc<green::Node>> for NodeOrToken<Arc<green::Node>, Arc<green::Token>> {
    fn from(this: Arc<green::Node>) -> Self {
        NodeOrToken::Node(this)
    }
}

impl From<Arc<green::Token>> for NodeOrToken<Arc<green::Node>, Arc<green::Token>> {
    fn from(this: Arc<green::Token>) -> Self {
        NodeOrToken::Token(this)
    }
}

impl<'a> From<&'a green::Node> for NodeOrToken<&'a green::Node, &'a green::Token> {
    fn from(this: &'a green::Node) -> Self {
        NodeOrToken::Node(this)
    }
}

impl<'a> From<&'a green::Token> for NodeOrToken<&'a green::Node, &'a green::Token> {
    fn from(this: &'a green::Token) -> Self {
        NodeOrToken::Token(this)
    }
}

impl<'a> From<&'a NodeOrToken<Arc<green::Node>, Arc<green::Token>>>
    for NodeOrToken<&'a green::Node, &'a green::Token>
{
    fn from(this: &'a NodeOrToken<Arc<green::Node>, Arc<green::Token>>) -> Self {
        this.as_deref()
    }
}

impl<'a> From<&'a Arc<green::Node>> for NodeOrToken<&'a green::Node, &'a green::Token> {
    fn from(this: &'a Arc<green::Node>) -> Self {
        NodeOrToken::Node(&*this)
    }
}

impl<'a> From<&'a Arc<green::Token>> for NodeOrToken<&'a green::Node, &'a green::Token> {
    fn from(this: &'a Arc<green::Token>) -> Self {
        NodeOrToken::Token(&*this)
    }
}

impl<'a> From<ArcBorrow<'a, green::Node>> for NodeOrToken<&'a green::Node, &'a green::Token> {
    fn from(this: ArcBorrow<'a, green::Node>) -> Self {
        NodeOrToken::Node(ArcBorrow::downgrade(this))
    }
}

impl<'a> From<ArcBorrow<'a, green::Token>> for NodeOrToken<&'a green::Node, &'a green::Token> {
    fn from(this: ArcBorrow<'a, green::Token>) -> Self {
        NodeOrToken::Token(ArcBorrow::downgrade(this))
    }
}

impl<'a> From<NodeOrToken<ArcBorrow<'a, green::Node>, ArcBorrow<'a, green::Token>>>
    for NodeOrToken<&'a green::Node, &'a green::Token>
{
    fn from(this: NodeOrToken<ArcBorrow<'a, green::Node>, ArcBorrow<'a, green::Token>>) -> Self {
        this.map(ArcBorrow::downgrade, ArcBorrow::downgrade)
    }
}
