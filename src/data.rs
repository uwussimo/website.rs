#[derive(Clone, Copy, PartialEq)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub minutes: &'static str,
    pub excerpt: &'static str,
    pub body: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub summary: &'static str,
    pub status: &'static str,
}

pub const POSTS: &[Post] = &[
    Post {
        slug: "latency-kills-teams",
        title: "Latency kills teams",
        date: "2026-04-06",
        minutes: "3 min read",
        excerpt: "The hidden drag in product teams is not effort. It is the delay between signal, decision, and shipped correction.",
        body: &[
            "Every team says they want speed, but most teams measure the wrong part. They watch how long implementation takes while ignoring the waiting around it.",
            "Latency shows up as unclear ownership, slow review loops, meetings without decisions, and work that sits finished but unreleased. None of those look expensive on a roadmap. Together they decide whether momentum survives.",
            "The fix is usually smaller than a reorg: shorten the feedback loop, make decisions reversible by default, and keep work visible until it reaches users.",
        ],
    },
    Post {
        slug: "art-of-shipping",
        title: "The Art of Shipping",
        date: "2026-04-03",
        minutes: "8 min read",
        excerpt: "Shipping is not pushing code. It is reducing uncertainty in public, with enough taste to make the next move obvious.",
        body: &[
            "A shipped product teaches you something a polished draft cannot. The market reacts to concrete behavior, not to intent.",
            "Good shipping rhythm requires scope discipline. You protect the central promise of the work and cut everything that does not help users feel that promise sooner.",
            "The best teams treat launch as a learning mechanism, then use the response to sharpen what comes next.",
        ],
    },
    Post {
        slug: "choose-your-mold",
        title: "Choose your mold: The art of becoming who you meant to be.",
        date: "2026-03-24",
        minutes: "6 min read",
        excerpt: "You become the shape you repeatedly practice. Pick the constraints that make your better defaults unavoidable.",
        body: &[
            "Ambition is easy to declare and hard to operationalize. The practical question is what environment makes your intended behavior automatic.",
            "A mold is a set of constraints: what you read, who sees your work, how quickly you publish, and what you refuse to optimize for.",
            "Choose those constraints deliberately. Otherwise the default mold around you will choose on your behalf.",
        ],
    },
];

pub const PROJECTS: &[Project] = &[
    Project {
        name: "Oqim",
        summary: "A focused mobile reading and learning product built with Rust and Dioxus.",
        status: "building",
    },
    Project {
        name: "Product Notes",
        summary: "Short essays and operating notes from startup engineering work.",
        status: "writing",
    },
    Project {
        name: "Founder Tools",
        summary: "Small internal systems for turning team conversations into shipped work.",
        status: "shipping",
    },
];

pub fn post_by_slug(slug: &str) -> Option<Post> {
    POSTS.iter().find(|post| post.slug == slug).copied()
}
