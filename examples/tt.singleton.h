#pragma once

class []FILE_NAME_AS_TYPE[] {
	public:
		static []FILE_NAME_AS_TYPE[] * instance() {
			if (_instance == nullptr) {
				_instance = new []FILE_NAME_AS_TYPE[]();
			}

			return _instance;
		}
	private:
		[]FILE_NAME_AS_TYPE[]() {
		}

		static []FILE_NAME_AS_TYPE[] * _instance;
}
