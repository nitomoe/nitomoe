#ifndef __LOG_H__
#define __LOG_H__

#include <iostream>

enum LogLevel
{
    LOG_LEVEL_DEBUG,
    LOG_LEVEL_INFO,
    LOG_LEVEL_ERROR,
    LOG_LEVEL_WARNING
};

#define LOG_ERROR(M) Log::stream() << Log::header(LOG_LEVEL_ERROR) << M << std::endl;
#define LOG_INFO(M) Log::stream() << Log::header(LOG_LEVEL_INFO) << M << std::endl;
#ifdef ENABLE_DEBUG_MACRO
#define LOG_DEBUG(M) Log::stream() << Log::header(LOG_LEVEL_DEBUG) << M << std::endl;
#else
#define LOG_DEBUG(M)
#endif
#define LOG_WARNING(M) Log::stream() << Log::header(LOG_LEVEL_WARNING) << M << std::endl;

class Log
{
public:
    static std::ostream &stream();
    static std::string header(const LogLevel &level);
};

#endif