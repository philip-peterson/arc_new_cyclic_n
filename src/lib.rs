
#![feature(arc_new_cyclic)]

//! Functions like Arc::new_cyclic which work for a varied number of arguments
pub mod arc {
    use std::sync::{Arc, Weak};

    pub fn new_cyclic_2<A, B>(data_fn: impl FnOnce(&Weak<A>, &Weak<B>) -> (A, B)) -> (Arc<A>, Arc<B>) {
        let mut b: Option<Arc<B>> = None;

        let a = Arc::new_cyclic(|weak_a| {
            let mut a : Option<A> = None;
            b = Some(Arc::new_cyclic(|weak_b| {
                let (a2, b2) = data_fn(weak_a, weak_b);
                a = Some(a2);
                b2
            }));
            a.unwrap()
        });

        (a, b.unwrap())
    }

    pub fn new_cyclic_3<A, B, C>(data_fn: impl FnOnce(&Weak<A>, &Weak<B>, &Weak<C>) -> (A, B, C)) -> (Arc<A>, Arc<B>, Arc<C>) {
        let mut c: Option<Arc<C>> = None;
        let mut b: Option<Arc<B>> = None;

        let a = Arc::new_cyclic(|weak_a| {
            let mut a : Option<A> = None;
            b = Some(Arc::new_cyclic(|weak_b| {
                let mut b: Option<B> = None;
                c = Some(Arc::new_cyclic(|weak_c| {
                    let (a2, b2, c2) = data_fn(weak_a, weak_b, weak_c);
                    a = Some(a2);
                    b = Some(b2);
                    c2
                }));
                b.unwrap()
            }));
            a.unwrap()
        });

        (a, b.unwrap(), c.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::arc;

    #[test]
    fn arc_new_cyclic_2_immediate_upgrades_fail() {
        arc::new_cyclic_2(|weak_a, weak_b| {
            assert_eq!(weak_a.upgrade(), None);
            assert_eq!(weak_b.upgrade(), None);

            (1, 2)
        });
    }

    #[test]
    fn arc_new_cyclic_2_immediate_dereferences_pass() {
        let (a, b) = arc::new_cyclic_2(|weak_a, weak_b| {
            assert_eq!(weak_a.upgrade(), None);
            assert_eq!(weak_b.upgrade(), None);

            (1, 2)
        });

        assert_eq!(*a, 1);
        assert_eq!(*b, 2);
    }

    #[test]
    fn arc_new_cyclic_2_deferred_upgrades_fail() {
        let mut a = None;
        let mut b = None;

        let stored = arc::new_cyclic_2(|weak_a, weak_b| {
            a = Some(weak_a.clone());
            b = Some(weak_b.clone());

            (1, 2)
        });

        assert_eq!(*a.expect("Failed to store").upgrade().expect("Upgrade failed"), 1);
        assert_eq!(*b.expect("Failed to store").upgrade().expect("Upgrade failed"), 2);
        drop(stored);
    }

    #[test]
    fn arc_new_cyclic_3_immediate_upgrades_fail() {
        arc::new_cyclic_3(|weak_a, weak_b, weak_c| {
            assert_eq!(weak_a.upgrade(), None);
            assert_eq!(weak_b.upgrade(), None);
            assert_eq!(weak_c.upgrade(), None);

            (1, 2, 3)
        });
    }

    #[test]
    fn arc_new_cyclic_3_immediate_dereferences_pass() {
        let (a, b, c) = arc::new_cyclic_3(|weak_a, weak_b, weak_c| {
            assert_eq!(weak_a.upgrade(), None);
            assert_eq!(weak_b.upgrade(), None);
            assert_eq!(weak_c.upgrade(), None);

            (1, 2, 3)
        });

        assert_eq!(*a, 1);
        assert_eq!(*b, 2);
        assert_eq!(*c, 3);
    }

    #[test]
    fn arc_new_cyclic_3_deferred_upgrades() {
        let mut a = None;
        let mut b = None;
        let mut c = None;

        let stored = arc::new_cyclic_3(|weak_a, weak_b, weak_c| {
            a = Some(weak_a.clone());
            b = Some(weak_b.clone());
            c = Some(weak_c.clone());

            (1, 2, 3)
        });

        assert_eq!(*a.expect("Failed to store").upgrade().expect("Upgrade failed"), 1);
        assert_eq!(*b.expect("Failed to store").upgrade().expect("Upgrade failed"), 2);
        assert_eq!(*c.expect("Failed to store").upgrade().expect("Upgrade failed"), 3);
        drop(stored);
    }
}
