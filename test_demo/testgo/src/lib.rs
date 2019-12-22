#[cfg(test)]
mod MyMod{

#[test]
 fn findFriendList(){
      assert_eq!(100,100 );
 }

#[test]
fn sayHello(){

    println!("{}","hello test" );
}

}


#[cfg(test)]
mod Msgmod{

 #[test]
 fn sendMsg()
 {
    println!("sendmsg haha");
 }

}


