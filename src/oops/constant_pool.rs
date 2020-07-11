// Java 1.0.2
const ConstantUtf8               : u16 = 1 ;
// Java 1.0.2
const ConstantInteger            : u16 = 3 ;
// Java 1.0.2
const ConstantFloat              : u16 = 4 ;
// Java 1.0.2
const ConstantLong               : u16 = 5 ;
// Java 1.0.2
const ConstantDouble             : u16 = 6 ;
// Java 1.0.2
const ConstantClass              : u16 = 7 ;
// Java 1.0.2
const ConstantString             : u16 = 8 ;
// Java 1.0.2
const ConstantFieldRef           : u16 = 9 ;
// Java 1.0.2
const ConstantMethodRef          : u16 = 10;
// Java 1.0.2
const ConstantInterfaceMethodRef : u16 = 11;
// Java 1.0.2
const ConstantNameAndType        : u16 = 12;
// Java 7
const ConstantMethodHandle       : u16 = 15;
// Java 7
const ConstantMethodType         : u16 = 16;
// Java 7
const ConstantInvokeDynamic      : u16 = 18;
// Java 9
const ConstantModule             : u16 = 19;
// Java 9
const ConstantPackage            : u16 = 20;
// Java 11
const ConstantDynamic            : u16 = 17;

pub struct ConstantInfo {}

impl ConstantInfo {
    pub fn from(tag: u16) {
        println!("{}", tag);
    }
}

pub struct ConstantPool {}

impl ConstantPool {}