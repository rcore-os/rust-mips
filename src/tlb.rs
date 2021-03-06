//! read & write TLB items

#![allow(deprecated)]

use crate::instructions;
use crate::registers::cp0;

/// refers to one TLB entry
pub struct TLBEntry {
    pub entry_lo0: cp0::entry_lo::EntryLo,
    pub entry_lo1: cp0::entry_lo::EntryLo,
    pub entry_hi: cp0::entry_hi::EntryHi,
    pub page_mask: cp0::page_mask::PageMask,
}

#[deprecated(since = "0.2.0", note = "please use implementations in `TLBEntry`")]
pub fn clear_all_tlb() {
    let mmu_size = cp0::config::mmu_size();
    if mmu_size != 0 {
        clear_tlb(0, mmu_size);
    } else {
        clear_tlb(0, 63);
    }
}

#[deprecated(since = "0.2.0", note = "please use implementations in `TLBEntry`")]
pub fn clear_tlb(start: u32, end: u32) {
    cp0::entry_lo::write0_u32(0);
    cp0::entry_lo::write1_u32(0);
    cp0::entry_hi::write_u32(0);
    cp0::page_mask::write_u32(0);
    for i in start..end + 1 {
        cp0::index::write_u32(i);
        unsafe { instructions::tlbwi() };
    }
}

#[deprecated(since = "0.2.0", note = "please use implementations in `TLBEntry`")]
pub fn read_tlb(index: u32) -> TLBEntry {
    cp0::index::write_u32(index);
    unsafe { instructions::tlbr() };
    TLBEntry {
        entry_lo0: cp0::entry_lo::read0(),
        entry_lo1: cp0::entry_lo::read1(),
        entry_hi: cp0::entry_hi::read(),
        page_mask: cp0::page_mask::read(),
    }
}

#[deprecated(since = "0.2.0", note = "please use implementations in `TLBEntry`")]
pub fn write_tlb(entry: TLBEntry, index: u32) {
    cp0::entry_lo::write0(entry.entry_lo0);
    cp0::entry_lo::write1(entry.entry_lo1);
    cp0::entry_hi::write(entry.entry_hi);
    cp0::page_mask::write(entry.page_mask);
    cp0::index::write_u32(index);
    unsafe { instructions::tlbwi() };
}

#[deprecated(since = "0.2.0", note = "please use implementations in `TLBEntry`")]
pub fn write_tlb_random(entry: TLBEntry) {
    cp0::entry_lo::write0(entry.entry_lo0);
    cp0::entry_lo::write1(entry.entry_lo1);
    cp0::entry_hi::write(entry.entry_hi);
    cp0::page_mask::write(entry.page_mask);
    unsafe { instructions::tlbwr() };
}

impl TLBEntry {
    pub fn clear(start: u32, end: u32) {
        clear_tlb(start, end);
    }

    pub fn clear_all() {
        clear_all_tlb();
    }

    pub fn read(index: u32) -> TLBEntry {
        read_tlb(index)
    }

    pub fn write(self, index: u32) {
        write_tlb(self, index);
    }

    pub fn write_random(self) {
        write_tlb_random(self);
    }
}
