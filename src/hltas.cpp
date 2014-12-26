#include <cassert>
#include <fstream>
#include <future>
#include <iostream>
#include <string>

#include "hltas.hpp"

namespace HLTAS
{
	enum ErrorCode {
		OK = 0,
		FAILOPEN,
		FAILVER
	};

	static const std::string ErrorDescriptions[] =
	{
		"Failed to open the file.",
		"Failed to read the version."
	};

	void Input::Clear()
	{
		// If we're reading some file, wait for it to finish.
		if (FinishedReading.valid())
			FinishedReading.wait();

		properties.clear();
		frames.clear();
	}

	std::shared_future<int> Input::Open(const std::string& filename)
	{
		FinishedReading = std::async(&Input::OpenInternal, this, filename);
		return FinishedReading;
	}

	const std::string& Input::GetErrorDescription(int errorCode)
	{
		assert(errorCode > 0);
		return ErrorDescriptions[errorCode - 1];
	}

	int Input::OpenInternal(const std::string& filename)
	{
		properties.clear();
		frames.clear();

		std::ifstream file(filename);
		if (!file)
			return ErrorCode::FAILOPEN;

		// Read the version.
		std::string temp;
		std::getline(file, temp, ' ');
		if (file.fail() || temp != "version")
			return ErrorCode::FAILVER;
		std::getline(file, temp);
		if (file.fail() || temp.empty())
			return ErrorCode::FAILVER;
		try {
			version = std::stoi(temp);
		} catch (...) {
			return ErrorCode::FAILVER;
		}

		ReadProperties(file);
		ReadFrames(file);

		return ErrorCode::OK;
	}

	void Input::ReadProperties(std::ifstream& file)
	{

	}

	void Input::ReadFrames(std::ifstream& file)
	{
		
	}

	std::unordered_map<std::string, std::string>& Input::GetProperties()
	{
		if (FinishedReading.valid())
			FinishedReading.wait();

		return properties;
	}

	std::vector<Frame>& Input::GetFrames()
	{
		if (FinishedReading.valid())
			FinishedReading.wait();

		return frames;
	}
}
