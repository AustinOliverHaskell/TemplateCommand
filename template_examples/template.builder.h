/*
[]BANNER{*||| This file was generated with tt v[]VERSION[], do not hand modify}[]
*/

class some_other_class; 

class []FILE_NAME_AS_TYPE{+Builder}[] {
	public:
		[]FILE_NAME_AS_TYPE{+Builder}[]() { 

[]FOR_EACH_FILE_IN_DIR{h|||
			[]FILE_NAME_AS_TYPE{lower}[] = nullptr;
}[]
		}


[]FOR_EACH_FILE_IN_DIR{h|||
		[]THIS_FILES_NAME_AS_TYPE{+Builder}[] * add_item([]FILE_NAME_AS_TYPE[] * item) { this->[]FILE_NAME_AS_TYPE{lower}[] = item; return this; }
}[]

	some_other_class * build() {
		// Your builder code here. 
	}

	private:

	[]FOR_EACH_FILE_IN_DIR{h|||
		[]FILE_NAME_AS_TYPE[] * []FILE_NAME_AS_TYPE{lower}[];
	}[]
};
