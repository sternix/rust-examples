fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (v1, v2) = v.split_at(4);
    assert_eq!(v1, &[1, 2, 3, 4]);
    assert_eq!(v2, &[5, 6, 7, 8, 9]);

    if let Some((e, vx)) = v.split_first() {
        assert_eq!(e, &1);
        assert_eq!(vx, &[2, 3, 4, 5, 6, 7, 8, 9]);
    }

    if let Some((e, vx)) = v.split_last() {
        assert_eq!(e, &9);
        assert_eq!(vx, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    let mut iter = v.split(|x| x % 5 == 0);
    assert_eq!(iter.next().unwrap(), &[1, 2, 3, 4]);
    assert_eq!(iter.next().unwrap(), &[6, 7, 8, 9]);
    assert_eq!(iter.next().is_none(), true);

    let mut iter = v.split(|x| x % 2 == 0);
    assert_eq!(iter.next().unwrap(), &[1]);
    assert_eq!(iter.next().unwrap(), &[3]);
    assert_eq!(iter.next().unwrap(), &[5]);
    assert_eq!(iter.next().unwrap(), &[7]);
    assert_eq!(iter.next().unwrap(), &[9]);
    assert_eq!(iter.next().is_none(), true);

    let mut iter = v.splitn(2, |x| x % 2 == 0);
    assert_eq!(iter.next().unwrap(), &[1]);
    assert_eq!(iter.next().unwrap(), &[3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(iter.next().is_none(), true);

    let mut iter = v.rsplitn(2, |x| x % 2 == 0);
    assert_eq!(iter.next().unwrap(), &[9]);
    assert_eq!(iter.next().unwrap(), &[1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(iter.next().is_none(), true);

    let mut iter = v.chunks(2);
    assert_eq!(iter.next().unwrap(), &[1, 2]);
    assert_eq!(iter.next().unwrap(), &[3, 4]);
    assert_eq!(iter.next().unwrap(), &[5, 6]);
    assert_eq!(iter.next().unwrap(), &[7, 8]);
    assert_eq!(iter.next().unwrap(), &[9]);
    assert_eq!(iter.next().is_none(), true);

    let mut iter = v.windows(4);
    assert_eq!(iter.next().unwrap(), &[1, 2, 3, 4]);
    assert_eq!(iter.next().unwrap(), &[2, 3, 4, 5]);
    assert_eq!(iter.next().unwrap(), &[3, 4, 5, 6]);
    assert_eq!(iter.next().unwrap(), &[4, 5, 6, 7]);
    assert_eq!(iter.next().unwrap(), &[5, 6, 7, 8]);
    assert_eq!(iter.next().unwrap(), &[6, 7, 8, 9]);
    assert_eq!(iter.next().is_none(), true);
}
