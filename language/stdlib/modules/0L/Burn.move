address 0x1 {
module Burn {
  use 0x1::Wallet;
  use 0x1::FixedPoint32;
  use 0x1::Vector;
  use 0x1::LibraAccount;
  use 0x1::CoreAddresses;
  use 0x1::GAS::GAS;
  use 0x1::Debug::print;

  resource struct BurnPreference {
    is_burn: bool
  }

  resource struct DepositInfo {
    addr: vector<address>,
    deposits: vector<u64>,
    ratio: vector<FixedPoint32::FixedPoint32>,
  }

  public fun reset_ratios(vm: &signer) acquires DepositInfo {
    CoreAddresses::assert_libra_root(vm);
    print(&0x100);

    let list = Wallet::get_comm_list();
    print(&0x101);
    let len = Vector::length(&list);
    let i = 0;
    let global_deposits = 0;
    let deposit_vec = Vector::empty<u64>();
    print(&0x102);

    while (i < len) {
      print(&0x110);

      let addr = *Vector::borrow(&list, i);
      let cumu = LibraAccount::get_cumulative_deposits(addr);
      global_deposits = global_deposits + cumu;
      Vector::push_back(&mut deposit_vec, cumu);
      i = i + 1;
    };
    print(&0x103);

    let ratios_vec = Vector::empty<FixedPoint32::FixedPoint32>();
    while (i < len) {
      print(&0x120);

      let cumu = *Vector::borrow(&deposit_vec, i);
      let ratio = FixedPoint32::create_from_rational(cumu, global_deposits);
      Vector::push_back(&mut ratios_vec, ratio);
      i = i + 1;
    };
    print(&0x104);

    if (exists<DepositInfo>(0x0)) {
      let d = borrow_global_mut<DepositInfo>(0x0);
      d.addr = list;
      d.deposits = deposit_vec;
      d.ratio = ratios_vec;
    } else {
      move_to<DepositInfo>(vm, DepositInfo {
        addr: list,
        deposits: deposit_vec,
        ratio: ratios_vec,
      })
    }
  }

  fun get_address_list(): vector<address> acquires DepositInfo {
    *&borrow_global<DepositInfo>(0x0).addr
  }

  fun get_value(payee: address, value: u64): u64 acquires DepositInfo {
    let d = borrow_global<DepositInfo>(0x0);
    let (_, i) = Vector::index_of(&d.addr, &payee);
    let ratio = *Vector::borrow(&d.ratio, i);
    FixedPoint32::multiply_u64(value, ratio)
  }

  fun epoch_start_burn(vm: &signer, payer: address, value: u64) acquires DepositInfo, BurnPreference {
    if (exists<BurnPreference>(payer)) {
      if (borrow_global<BurnPreference>(payer).is_burn) {
        return burn(vm, payer, value)
      }
    };
    send(vm, payer, value);
  }

  fun burn(vm: &signer, payer: address, value: u64) {
      LibraAccount::vm_make_payment_no_limit<GAS>(
          payer,
          0xDEADDEAD,
          value,
          b"epoch start burn",
          b"epoch start burn",
          vm,
      );
  }


  fun send(vm: &signer, payer: address, value: u64) acquires DepositInfo {
    let list = get_address_list();
    let len = Vector::length<address>(&list);
    let i = 0;
    while (i < len) {
      let payee = *Vector::borrow<address>(&list, i);
      let val = get_value(payee, value);
      
      LibraAccount::vm_make_payment_no_limit<GAS>(
          payer,
          payee,
          val,
          b"epoch start send",
          b"epoch start send",
          vm,
      );
      i = i + 1;
    };
  }
  //////// GETTERS ////////
  public fun get_ratios(): (vector<address>, vector<u64>, vector<FixedPoint32::FixedPoint32>) acquires DepositInfo {
    let d = borrow_global<DepositInfo>(0x0);
    (*&d.addr, *&d.deposits, *&d.ratio)

  }

  //////// TEST HELPERS ////////
  
}
}
