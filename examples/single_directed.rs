use bastion_streams::stream::flow::map::Map;
use bastion_streams::stream::sink::ignore::Ignore;
use bastion_streams::stream::source::single::Single;

use bastion_streams::stream::topology::architect::Architect;
use bastion_streams::stream::topology::container::Container;

fn main() {
    let single = Single::<i32>::new(999);
    let mapper = Map::<i32, i32>::new(Box::new(|x: i32| x + 1));
    let sink = Ignore::<i32>::new();

    let stages = vec![
        Container::Source(Box::new(single)),
        Container::Transform(Box::new(mapper)),
        Container::Sink(Box::new(sink)),
    ];

    let mut architect = Architect::graph(stages);
    architect.check_bounds();
    architect.visit_stages();
    architect.run();
}
