// SPDX-License-Identifier: MIT

#include "public/EventBuffer.h"

#include <elos/event/event.h>
#include <elos/eventfilter/eventfilter.h>
#include <elos/eventfilter/eventfilter_types.h>
#include <safu/log.h>
#include <safu/result.h>
#include <safu/vector.h>

EventBuffer::EventBuffer(size_t len)
    : size(len), start(0), end(SIZE_MAX), buffer(new elosEvent_t[len]) {
        for (size_t idx = 0; idx < size; idx++) {
            elosEventInitialize(&this->buffer[idx]);
        }
    }
EventBuffer::~EventBuffer() {
    for (size_t idx = 0; idx < this->size; idx++) {
        elosEventDeleteMembers(&this->buffer[idx]);
    }
}
size_t EventBuffer::indexIncrement(size_t idx) const {
    return (idx + 1) % this->size;
}
safuResultE_t EventBuffer::pushEvent(const elosEvent_t *event) {
    safuResultE_t result = SAFU_RESULT_OK;

    if (SIZE_MAX == this->end) {
        result = elosEventDeepCopy(&this->buffer[this->start], event);
        this->end = 1;
        return result;
    }
    result = elosEventDeleteMembers(&this->buffer[this->end]);
    if (result != SAFU_RESULT_OK) {
        safuLogErr("Failed to free event from buffer");
    }
    result = elosEventDeepCopy(&this->buffer[this->end], event);

    if (this->start == this->end) {
        this->start = this->indexIncrement(this->start);
    }
    this->end = this->indexIncrement(end);

    return result;
}
safuResultE_t EventBuffer::findEvents(const elosRpnFilter_t *filter,
        safuVec_t *eventList) const {
    safuResultE_t result = SAFU_RESULT_OK;
    if (SIZE_MAX == this->end) {
        safuLogDebug("EventRingBuffer is empty!");
        return result;
    }
    size_t idx = this->start;
    do {
        elosRpnFilterResultE_t filterResult;
        filterResult = elosEventFilterExecute(filter, nullptr, &this->buffer[idx]);
        if (filterResult == RPNFILTER_RESULT_MATCH) {
            auto *event = new elosEvent_t();
            elosEventDeepCopy(event, &this->buffer[idx]);
            safuVecPush(eventList, event);
        } else if (filterResult == RPNFILTER_RESULT_ERROR) {
            safuLogErr("Error fetching event from in memory backend!");
            result = SAFU_RESULT_FAILED;
        }
        idx = this->indexIncrement(idx);
    } while (idx != this->end);

    return result;
}
