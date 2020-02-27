<template>
  <div class="d-flex flex-grow-1">
    <v-card class="flex-grow-1 ma-5">
      <v-list-item-title class="headline ma-3">Bank 0</v-list-item-title>
      <v-list>
        <v-simple-table>
          <template v-slot:default>
            <tbody>
              <tr v-for="item in bank0" :key="item.index">
                <td>{{item.name}}</td>
                <td align="right">{{hexString(item.value)}}</td>
              </tr>
            </tbody>
          </template>
        </v-simple-table>
      </v-list>
    </v-card>
    <v-card class="flex-grow-1 ma-5">
      <v-list-item-title class="headline ma-3">Bank 1</v-list-item-title>
      <v-list>
        <v-simple-table>
          <template v-slot:default>
            <tbody>
              <tr v-for="item in bank1" :key="item.index">
                <td>{{item.name}}</td>
                <td align="right">{{hexString(item.value)}}</td>
              </tr>
            </tbody>
          </template>
        </v-simple-table>
      </v-list>
    </v-card>
  </div>
  <!--<v-data-table :headers="headers" :items="items" class="sfr-viewer elevation-1"></v-data-table>-->
</template>
</template>

<script>
export default {
  name: "SFRViewer",
  props: ["bank"],
  data() {
    return {
      bank0: null,
      bank1: null
    };
  },
  mounted() {
    this.bank0 = this.getBank0();
    this.bank1 = this.getBank1();
  },
  methods: {
    hexString(value) {
      let str = value.toString(16);

      if (str.length < 2) {
        str = "0" + str;
      }

      return str;
    },
    getBank0() {
      return [
        {
          name: "INDIRECT",
          value: this.bank.indirect
        },
        {
          name: "TMR0",
          value: this.bank.tmr0
        },
        {
          name: "PCL",
          value: this.bank.pcl
        },
        {
          name: "STATUS",
          value: this.bank.status
        },
        {
          name: "FSR",
          value: this.bank.fsr
        },
        {
          name: "PORTA",
          value: this.bank.porta
        },
        {
          name: "PORTB",
          value: this.bank.portb
        },
        {
          name: "EEDATA",
          value: this.bank.eedata
        },
        {
          name: "EEADR",
          value: this.bank.eeadr
        },
        {
          name: "PCLATH",
          value: this.bank.pclath
        },
        {
          name: "INTCON",
          value: this.bank.intcon
        }
      ];
    },
    getBank1() {
      return [
        {
          name: "INDIRECT",
          value: this.bank.indirect
        },
        {
          name: "OPTION",
          value: this.bank.option
        },
        {
          name: "PCL",
          value: this.bank.pcl
        },
        {
          name: "STATUS",
          value: this.bank.status
        },
        {
          name: "FSR",
          value: this.bank.fsr
        },
        {
          name: "TRISA",
          value: this.bank.trisa
        },
        {
          name: "TRISB",
          value: this.bank.trisb
        },
        {
          name: "EECON1",
          value: this.bank.eecon1
        },
        {
          name: "EECON2",
          value: this.bank.eecon2
        },
        {
          name: "PCLATH",
          value: this.bank.pclath
        },
        {
          name: "INTCON",
          value: this.bank.intcon
        }
      ];
    }
  },
  watch: {
    bank: function(val) {
      this.bank0 = this.getBank0();
    }
  }
};
</script>
