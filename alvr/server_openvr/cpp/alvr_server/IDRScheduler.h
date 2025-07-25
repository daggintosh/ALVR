#pragma once

#include "Settings.h"
#include <mutex>
#include <stdint.h>

class IDRScheduler {
public:
    IDRScheduler();
    ~IDRScheduler();

    void OnStreamStart();
    void InsertIDR();

    bool CheckIDRInsertion();

private:
    static const int MIN_IDR_FRAME_INTERVAL = 100 * 1000; // 100-milliseconds
    uint64_t m_insertIDRTime = 0;
    bool m_scheduled = false;
    std::mutex m_mutex;
    uint64_t m_minIDRFrameInterval = MIN_IDR_FRAME_INTERVAL;
};
