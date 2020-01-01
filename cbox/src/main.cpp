#include <stdio.h>

#define derefImpl(T) \
   public:\
    T& operator *(){\
       return *v;\
    }
template <class T>
class Box{
 T *v;
 public:
    Box(T * t){
        this->v = t;
    }
  
    ~Box(){
        if (v!=0)
        {
            delete v;
            v= 0;
        }
    }
      derefImpl(T)
};
template <class T>
class Arc{
 T *v;
  public:
     Arc(T * t){
        this->v = t;
    }
  derefImpl(T)
};


class Student{
   public:
     void say(){
         printf("hello world");
     }
};

int main(int argc, char const *argv[])
{
     //ok
    // Arc<Student> *arc = new Arc<Student>(new Student());
    // (**arc).say();
    //ok

//嵌套式
    Box<Arc<Student> > b(new Arc<Student>(new Student()));
     
     (**b).say();
    
    return 0;
}
