// See https://betterprogramming.pub/rust-events-revisited-926486721e3f

#[macro_export]
macro_rules! signal {
  ($sig:ident<$rectype:ident, $data:ident>) => {
    pub struct $sig {
      counter: usize,
      recs: Vec<$rectype>,
    }

    #[derive(Copy, Clone)]
    pub struct $rectype {
      id: usize,
      cls: fn($rectype, $data),
    }

    impl $rectype {
      fn new(id: usize, cls: fn($rectype, $data)) -> Self {
        Self { id, cls }
      }
    }

    impl EventReceiver for $rectype {
      type Data = $data;
      type Transformer = fn($rectype, $data);

      fn on_emit(self, data: Self::Data) {
        (self.cls)(self, data);
      }
    }

    impl $sig {
      fn new() -> Self {
        Self {
          counter: 0,
          recs: Vec::new(),
        }
      }

      fn nxt(&mut self) -> usize {
        self.counter += 1;
        self.counter
      }
    }

    impl Signal for $sig {
      type Data = $data;
      type RecType = $rectype;

      fn emit(&self, data: Self::Data) {
        self.recs.iter().for_each(|r| r.on_emit(data.clone()));
      }

      fn connect(
        &mut self,
        transformer: <Self::RecType as EventReceiver>::Transformer,
      ) -> Self::RecType {
        let id = self.nxt();
        let rec = $rectype::new(id, transformer);
        self.recs.push(rec);
        rec
      }

      fn disconnect(&mut self, id: usize) {
        self.recs.retain(|rec| rec.id != id);
      }
    }
  };
}

pub trait WSRegister {
  type Data;
}

pub trait EventReceiver {
  type Data;
  type Transformer;

  fn on_emit(self, data: Self::Data);
}

pub trait Signal {
  type Data;
  type RecType: EventReceiver;

  fn emit(&self, data: Self::Data);
  fn connect(&mut self, rec: <Self::RecType as EventReceiver>::Transformer) -> Self::RecType;
  fn disconnect(&mut self, i: usize);
}

#[cfg(test)]
mod tests {
  use crate::signals::{EventReceiver, Signal};

  #[derive(Copy, Clone)]
  pub struct MySignalData {
    num: i32,
  }

  #[test]
  fn test_signal() {
    signal!(TextSignal<TextReceiver, MySignalData>);
    let mut text_emitter = TextSignal::new();
    let rec1 = text_emitter.connect(|this, data| {
      println!("Receiver ms R{} - num: {}", this.id, data.num);
    });

    text_emitter.emit(MySignalData { num: 5 });
    text_emitter.disconnect(rec1.id);
    let _rec3 = text_emitter.connect(|this, data| {
      println!("Receiver ms R{} - num: {}", this.id, data.num);
    });
  }
}
